///////////////////////////////////////////////////////////////////////////////
//
//   El programa recibe como parámetro en la línea de comandos un fichero .pcap
//   con paquetes capturados
//
//   El programa procesa cada paquete del fichero, mostrando en la salida
//   estándar todos los bytes del paquete y a continuación sus cabeceras
//   protocolo a protocolo: Ethernet, ARP, IP, ICMP, TCP y UDP
//
//   Los paquetes ARP son recolectados en un Vec y al final se guardan
//   en un nuevo fichero .pcap
//
///////////////////////////////////////////////////////////////////////////////


use std::collections::HashMap;//colección para usar de las bibliotecas de Rust: HashMap(Clave, valor)
use std::env; //permite acceder a la linea de comandos
use std::net::Ipv4Addr; //permite trabajar con el protocolo IPv4


use pcap::Capture; //permite trabajar con capturas de tráfico


use colored::*; //colorea la salida de otro color

//para trabajar con los paquetes de los distintos protocolos
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;


fn display_eth_frame(ethernet_packet: &EthernetPacket) {
    println!(
        "{}",
        format!(
            "Ethernet frame: {} -> {}",
            ethernet_packet.get_source(),
            ethernet_packet.get_destination()
        )
    );
}


fn display_ipv4_packet(ipv4_packet: &Ipv4Packet) {
    println!(
        "    {}",
        format!(
            "IP Datagram: {} -> {}, ttl: {}",
            ipv4_packet.get_source(),
            ipv4_packet.get_destination(),
            ipv4_packet.get_ttl(),
        )
            .yellow()
    );
}


fn display_tcp_packet(tcp_packet: &TcpPacket, ipv4_packet: &Ipv4Packet) {
    let mut sflags = String::from("");
    let flags = tcp_packet.get_flags();
    if (flags & 0b0010_0000) != 0 {
        sflags.push_str(" URG");
    }
    if (flags & 0b0001_0000) != 0 {
        sflags.push_str(" ACK");
    }
    if (flags & 0b0000_1000) != 0 {
        sflags.push_str(" PSH");
    }
    if (flags & 0b0000_0100) != 0 {
        sflags.push_str(" RST");
    }
    if (flags & 0b0000_0010) != 0 {
        sflags.push_str(" SYN");
    }
    if (flags & 0b0000_0001) != 0 {
        sflags.push_str(" FIN");
    }


    println!(
        "        {}",
        format!(
            "TCP Packet: {}:{} -> {}:{}; Flags: {}",
            ipv4_packet.get_source(),
            tcp_packet.get_source(),
            ipv4_packet.get_destination(),
            tcp_packet.get_destination(),
            sflags
        )
            .bright_blue()
    );
}


#[derive(Debug)] //es un atributo que permite implementar de forma automática el trait debug, para pintar de forma más limpia en la salida

struct SynScannerData {  //almacena los datos relacionados con un escaner SYN
    syn_segments_send: u32,  //número de SYN mandados por la dirección IP
    syn_ack_segments_receive: u32 //número de segmentos SYN+ACK recibidos por esa dirección IP
}


fn main() -> Result<(), String> {
    //Me creo la variable mutable synScanner, que es un HashMap, de clave su dirección IP;
    // valor, el struct de info del SynScanner{syn_segments_send, syn_ack_segments_receive}
    let mut syn_scanners: HashMap<Ipv4Addr, SynScannerData> = HashMap::new();


    // Obtiene en file_name el nombre de fichero del primer argumento del programa al ejecutarlo
    let Some(file_name) = env::args().nth(1) else {
        return Err(String::from("Tienes que especificar en la línea de comandos el nombre del fichero pcap."))
    };


    // Devuelve en cap un iterador para los paquetes del fichero pcap con nombre file_name
    let Ok(mut cap) = Capture::from_file(&file_name) else {
        return Err(format!("El fichero {} no existe o no contiene una captura pcap.", file_name))
    };


    // Procesa cada paquete contenido en cap, almacenando en packet el siguiente paquete de cap, y para cada uno:
    //   Muestra en stdio el paquete pcap (sus bytes)
    //   Muestra en stdio cada cabecera de las que tenga el paquete (Ethernet, ARP, IP, ICMP, TCP, UDP)
    //   Si el paquete es un paquete ARP se almacena en el Vec arp_packets
    while let Ok(packet) = cap.next_packet() {
        //println!("\n---\nPaquete: {:?}\n---", packet);


        // Si el paquete es una trama Ethernet, construye un pnet::EthernetPacket a partir de
        // los bytes de packet que están en packet.data
        if let Some(ethernet_packet) = EthernetPacket::new(&packet.data) {
            //display_eth_frame(&ethernet_packet);


            match ethernet_packet.get_ethertype() {
                EtherTypes::Ipv4 => {
                    // Si la trama Ethernet contiene un datagrama IP, construye un pnet::Ipv4Packet con
                    // el campo de datos de la trama ethernet
                    if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                        //display_ipv4_packet(&ipv4_packet);


                        match ipv4_packet.get_next_level_protocol() {
                            IpNextHeaderProtocols::Tcp => {
                                // Si el datagrama IP contiene un segmento TCP, construye un pnet::TcpPacket
                                // con el campo de datos del datagrama IP
                                if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                    //display_tcp_packet(&tcp_packet, &ipv4_packet);

                                //para comprobar las flags, (SYN) o (SYN + ACK)
                                    let flags = tcp_packet.get_flags();
                                    if (flags & 0b0000_0010) != 0 && (flags & 0b0001_0000) == 0 { // SYN && NOT ACK SEND
                                        syn_scanners.entry(ipv4_packet.get_source()).and_modify(|ssd| ssd.syn_segments_send += 1).or_insert(SynScannerData { syn_segments_send: 1, syn_ack_segments_receive: 0 });
                                    } //entrar en el HashMap syn_scanners: comprobar si está esa dirección IP, si lo está incrementar en 1 el syn_segments_send
                                    //si no está esa entrada, crear una entrada con esa clave, dirección IP;  syn_segments_send: 1, syn_ack_segments_receive: 0 (VALOR)


                                    if (flags & 0b0000_0010) != 0 && (flags & 0b0001_0000) != 0 { // SYN + ACK RECEIVED
                                        syn_scanners.entry(ipv4_packet.get_destination()).and_modify(|ssd| ssd.syn_ack_segments_receive += 1).or_insert(SynScannerData { syn_segments_send: 0, syn_ack_segments_receive: 1 });
                                    } //entrar en el HashMap syn_scanners: comprobar si está esa dirección IP, si lo está incrementar en 1 el syn_segments_receive
                                    ////si no está esa entrada, crear una entrada con esa clave, dirección IP;  syn_segments_send: 0, syn_ack_segments_receive: 1 (VALOR)
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }


    for (ip, SynScannerData { syn_segments_send, syn_ack_segments_receive }) in syn_scanners {
        //print!("{} | {} | {}", ip, syn_segments_send, syn_ack_segments_receive);
        if syn_segments_send > 5 && syn_segments_send > (syn_ack_segments_receive * 3) { //algoritmo para detectar SYNScanners
            println!("{}", ip);
        }
    }


    return Ok(())
} // Al terminar el ámbito en el que se declara arp_file se hace drop, lo que cierra el fichero

