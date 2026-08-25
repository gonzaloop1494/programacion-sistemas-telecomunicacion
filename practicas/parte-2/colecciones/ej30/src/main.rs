use std::env;
use std::net::Ipv4Addr;


use colored::Colorize;
use pcap::Capture;


use pnet::packet::arp::{ArpOperations, ArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;
use pnet::util::MacAddr;


use std::collections::{HashMap, HashSet};


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


fn main() -> Result<(), String> {
    let mut arp_requests: HashMap<Ipv4Addr, HashSet<MacAddr>> = HashMap::new();
    let mut unsolicited_replies: HashMap<MacAddr, u32> = HashMap::new();


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
                EtherTypes::Arp => {
                    // Si la trama Ethernet contiene un paquete es un ARP, construye un pnet::ArpPacket con
                    // el campo de datos de la trama ethernet
                    if let Some(arp_packet) = ArpPacket::new(ethernet_packet.payload()) {
                        //display_arp_packet(&arp_packet);


                        match arp_packet.get_operation() {
                            ArpOperations::Request => {
                                let target_ip = arp_packet.get_target_proto_addr();
                                let sender_mac = arp_packet.get_sender_hw_addr();
                                arp_requests.entry(target_ip).or_insert(HashSet::new()).insert(sender_mac);
                            }
                            ArpOperations::Reply => {
                                let sender_ip = arp_packet.get_sender_proto_addr();
                                let sender_mac = arp_packet.get_sender_hw_addr();
                                let requesting_mac = arp_packet.get_target_hw_addr();
                                if let Some(requesting_macs) = arp_requests.get_mut(&sender_ip) {
                                    if requesting_macs.remove(&requesting_mac) {
                                        if requesting_macs.is_empty() {
                                            arp_requests.remove(&sender_ip);
                                        }
                                    } else {
                                        unsolicited_replies.entry(sender_mac).and_modify(|v| { *v += 1; }).or_insert(1);
                                    }
                                } else {
                                    unsolicited_replies.entry(sender_mac).and_modify(|v| { *v += 1; }).or_insert(1);
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


    //println!("ARP requests: {:?}", arp_requests);
    //println!("Unsolicited replies: {:?}", unsolicited_replies);
    for (mac, count) in unsolicited_replies {
        if count > 5 {
            println!("{}", mac);
        }
    }


    return Ok(())
}

