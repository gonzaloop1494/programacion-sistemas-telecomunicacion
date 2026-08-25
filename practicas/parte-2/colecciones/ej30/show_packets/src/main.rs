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

use std::env;

use pcap::Capture;

use colored::*;

use pnet::packet::arp::ArpPacket;
use pnet::packet::arp::ArpOperations;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
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

fn display_arp_packet(arp_packet: &ArpPacket) {
    match arp_packet.get_operation() {
        ArpOperations::Request => {
            println!(
                "    {}",
                format!(
                    "ARP Request Packet: {} asking for {} eth addr",
                    arp_packet.get_sender_hw_addr(),
                    arp_packet.get_target_proto_addr()
                )
                .red()
            )
        }
        ArpOperations::Reply => {
            println!(
                "    {}",
                format!(
                    "ARP Reply Packet: {} is at {}",
                    arp_packet.get_sender_proto_addr(),
                    arp_packet.get_sender_hw_addr(),
                )
                .bright_red()
            )
        }
        _ => {}
    }
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

fn display_icmp_packet(icmp_packet: &IcmpPacket) {
    println!(
        "        {}",
        format!(
            "ICMP Packet: {:?}, {:?}",
            icmp_packet.get_icmp_type(),
            icmp_packet.get_icmp_code()
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

fn display_udp_packet(udp_packet: &UdpPacket, ipv4_packet: &Ipv4Packet) {
    println!(
        "        {}",
        format!(
            "UDP Packet: {}:{} -> {}:{}",
            ipv4_packet.get_source(),
            udp_packet.get_source(),
            ipv4_packet.get_destination(),
            udp_packet.get_destination(),
        )
        .green()
    );
}

fn main() -> Result<(), String> {
    // Vec para almacenar paquetes ARP en forma de tupla (cabecera pcap, Vec de los bytes del paquete)
    //    nota: pcap::PacketHeader es la cabecera que añade el formato pcap a cada paquete de 
    //    una captura (con el timestamp y el tamaño del paquete), no es una cabecera de un protocolo 
    let mut arp_packets: Vec<(pcap::PacketHeader, Vec<u8>)> = Vec::new();

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
        println!("\n---\nPaquete: {:?}\n---", packet);

        // Si el paquete es una trama Ethernet, construye un pnet::EthernetPacket a partir de 
        // los bytes de packet que están en packet.data
        if let Some(ethernet_packet) = EthernetPacket::new(&packet.data) {
            display_eth_frame(&ethernet_packet);

            match ethernet_packet.get_ethertype() {
                EtherTypes::Arp => {
                    // Si la trama Ethernet contiene un paquete es un ARP, construye un pnet::ArpPacket con 
                    // el campo de datos de la trama ethernet
                    if let Some(arp_packet) = ArpPacket::new(ethernet_packet.payload()) {
                        display_arp_packet(&arp_packet);

                        // inserta el paquete pcap en el Vec arp_packets en forma de tupla (cabecera, bytes_de_datos)
                        arp_packets.push((packet.header.clone(), packet.data.to_vec()));
                    }
                }
                EtherTypes::Ipv4 => {
                    // Si la trama Ethernet contiene un datagrama IP, construye un pnet::Ipv4Packet con
                    // el campo de datos de la trama ethernet
                    if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                        display_ipv4_packet(&ipv4_packet);

                        match ipv4_packet.get_next_level_protocol() {
                            IpNextHeaderProtocols::Icmp => {
                                // Si el datagrama IP contiene un paquete ICMP, construye un pnet::IcmpPacket
                                // con el campo de datos del datagrama IP
                                if let Some(icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
                                    display_icmp_packet(&icmp_packet);
                                }
                            }
                            IpNextHeaderProtocols::Tcp => {
                                // Si el datagrama IP contiene un segmento TCP, construye un pnet::TcpPacket
                                // con el campo de datos del datagrama IP
                                if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                    display_tcp_packet(&tcp_packet, &ipv4_packet);
                                }
                            }
                            IpNextHeaderProtocols::Udp => {
                                // Si el datagrama IP contiene un datagrama UDP, construye un pnet::UdpPacket
                                // con el campo de datos del datagrama IP
                                if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                    display_udp_packet(&udp_packet, &ipv4_packet);
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

    // Crea arp_file como un manejador de un nuevo fichero pcap en el 
    // que se pueden escribir paquetes pcap 
    let mut arp_file = Capture::dead(pcap::Linktype::ETHERNET)
        .unwrap()
        .savefile(String::from("arp_file.pcap"))
        .unwrap();

    // Escribe cada paquete de arp_packets en arp_file
    for p in arp_packets {
        let packet = pcap::Packet::new(&p.0, &p.1);
        arp_file.write(&packet);
    }

    // Se asegura de que todos los bytes escritos se vuelcan en el fichero
    arp_file.flush().unwrap();

    return Ok(())
} // Al terminar el ámbito en el que se declara arp_file se hace drop, lo que cierra el fichero
