use std::net::UdpSocket; //proporciona funcionalidad para comunicaciones sobre UDP
use std::net::SocketAddr; // representa direcciones de sockets,
// construidas a partir de una dirección IP y un puerto, con sus variantes SocketAddrV4 y SocketAddrV6.
use std::net::Ipv4Addr;//representa direcciones IP, con sus variantes Ipv4Addr e Ipv6Addr.
use std::str;

fn main() -> std::io::Result<()> {
    // Abre un socket UDP para cualquier IP local, y un puerto libre cualquiera
    // Equivalente a: let socket = UdpSocket::bind("0.0.0.0:0")?;
    let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))?;
    println!(
        "Abierto socket en IP: {}, Puerto: {}",
        socket.local_addr()?.ip(),
        socket.local_addr()?.port());

    // String a enviar
    let m = String::from("¡Hola! Este es un mensaje muy largo para colapsar y llenar tu buffer.");

    // Convierte el String a un array de bytes y lo envía a un servidor en "127.0.0.1:8888"
    socket.send_to(m.as_bytes(), "127.0.0.1:8888")?;

    // Buffer para recibir la respuesta, suficientemente grande
    let mut buffer = [0u8; 1500];

    // Recibe un datagrama UDP que llegue al socket
    let (len, server) = socket.recv_from(&mut buffer)?;

    // Convierte los datos a &str con str::from_utf8(&buffer)
    println!(
        "Recibido mensaje de longitud {len} del servidor {server}: {}",
        str::from_utf8(&buffer).expect("El mensaje no se puede convertir a string")
    );

    Ok(())
}
