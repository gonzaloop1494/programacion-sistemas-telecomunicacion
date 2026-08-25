use std::env;

use pcap::Capture; //crate para leer y escribir ficheros de captura de paquetes Ethernet

use pnet::packet::arp;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;
use pnet::util::MacAddr; //se utiliza para representar direcciones Ethernet
use core::net::Ipv4Addr; //se utiliza para representar direcciones IPv4

// colecciones que voy a utilizar en el programa
use std::collections::{Hashmap, HashSet};

fn main() -> Result<(), String> {
//me creo una colección HashMap para almacenar las solicitudes ARP
    let mut arp_requests: HashMap<Ipv4Addr, HashSet<MacAddr>> = HashMap::new(); //me creo la variable mutable, que es una colección HashMap
    //cuya clave es la dirección IP de la cual pregunto por su dirección Ethernet; y el valor es un conjunto de valores que representan
    //las direcciones Ethernet que envían la solicitud ARP
//Me creo una colección HashMap para almacenar las respuestas enviadas sin solicitud, con clave dirección Ethernet del que las envía, y valor
// que va a ser un contador del número de veces que la envía
// cuando envíe más de 5, será considerado un ARP spoofer
    let mut unsolicited_replies: HashMap<MacAddr, u32> = HashMap::new(); //en caso de ser ARP spoofer, se imprime esa MacAddr

//Obtiene en file_name el nombre de fichero del primer argumento del programa al ejecutarlo
    let Some(file_name) =env::args().nth(1) else { // utiliza el módulo env para acceder
        // a los argumentos de la línea de comandos.
        // El método args() devuelve un iterador sobre los argumentos pasados al programa.
        // La función nth(1) obtiene el segundo argumento de la lista
        return Err(String::from("Tienes que especificar en la línea de comandos el nombre del fichero pcap. "))
    };

    //Devuelve en cap un iterador para los paquetes del fichero pcap con nombre file_name
    let Ok(mut cap) = Capture::from_file(&file_name) else {
        return Err(format! ("El fichero {} no existe o no contiene una captura pcap. ", file_name))
    };

    //Procesa cada paquete contenido en cap, almacenando en packet el siguiente paquete de cap, y para cada uno muestra:
    //  en stdio el paquete (sus bytes)
    //  en stdio cada cabecera que contenga el paquete (Ethernet ARP, IP ICMP, TCP, UDP)
    //  Si el paquete es un paquete ARP se almacena dependiendo de la operación
    //Reply o Request, en el HashMap arp_requests o el HashMap unsolicited_replies
    while let Ok(packet) = cap.next_packet() {
        println!("\n---\nPaquete: {:?}\n---", packet);

        // Si el paquete es una trama Ethernet, construye un pnet::EthernetPacket a partir de
        // los bytes de packet que están en el packet.data
        if let Some(ethernet_packet) = EthernetPacket::new(&packet.data) {
            //display_eth_frame(&ethernet_packet); -> habría que crear la función para mostrar la trama Ethernet
            match ethernet_packet.get_ethertype() {
                EtherTypes::Arp => {
                    // Si la trama Ethernet contiene un paquete es un ARP, construye un pnet::ArpPacket con
                    // el campo de datos de la trama Ethernet
                    if let Some(arp_packet) = ArpPacket::new(ethernet_packet.payload()) {
                        //display_arp_packet(&arp_packet); -> habría que crear la función para mostrar el paquete arp

                        //AHORA HAY QUE DIFERENCIAR EL TIPO DE OPERACIÓN EN ARP: REQUEST O REPLY
                        match arp_packet.get_operation() {
                            ArpOperations::Request => {
                                let target_ip: Ipv4addr = arp_packet.get_target_ip_addr(); //crear variable para dirección IP del que recibe la solicitud ARP
                                let sender_mac: MacAddr = arp_packet.get_sender_hw_addr(); //crear variable para dirección Ethernet del que envía la solcitud ARP
                                arp_requests.entry(target_ip).or_insert(HashSet::new()).insert(sender_mac);
                                //Si la entrada del HashMap contiene o no algo
                            }
                            ArpOperations::Reply => {
                                let sender_ip: Ipv4Addr = arp_packet.get_sender_ip_addr(); //variable para la dirección IP del que envía la respuesta ARP
                                let sender_mac: MacAddr = arp_packet.get_sender_mac_addr(); //variable para la dirección Ethernet del que envía la respuesta ARP
                                                                                            //Es la que solicitaban con la petición
                                let requesting_mac: MacAddr = arp_packet.get_target_hw_addr(); //variable para la dirección Ethernet del que recibe la respuesta ARP
                                if let Some(requesting_macs) = arp_requests.get_mut(&sender_ip) { //comprueba si el requesting_mac ha solicitado la dirección Ethernet
                                                                                                  //de la dirección IP sender_ip
                                    if requesting_macs.remove(&requesting_mac) { //intenta eliminar requesting_mac de la colección requesting_macs.
                                                                                // Si la eliminación es exitosa (es decir, si requesting_mac estaba presente),
                                                                                // se entra al siguiente bloque.
                                        if requesting_macs.is_empty() {
                                            arp_requests.remove(&sender_ip);
                                        }

                                    } else {
                                        unsolicited_replies.entry(sender_mac).and_modify(|v| { *v += 1;}).or_insert(1);
                                    }


                                } else {
                                    unsolicited_replies.entry(sender_mac).and_modify(|v| { *v += 1;}).or_insert(1);

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
    println!("solicitudes ARP: {:?}", arp_requests);
    println!("Unsolicited replies: {:?}", unsolicited_replies);
    for (mac, count) in unsolicited_replies {
        if count > 5 {
            println!("{}", mac);
        }
    }
    return Ok(());

}
