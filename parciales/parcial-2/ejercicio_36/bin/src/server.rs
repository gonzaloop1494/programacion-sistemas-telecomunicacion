use std::net::UdpSocket; //proporciona funcionalidad para comunicaciones sobre UDP
use std::net::SocketAddr; // representa direcciones de sockets,
// construidas a partir de una dirección IP y un puerto, con sus variantes SocketAddrV4 y SocketAddrV6.
use std::net::Ipv4Addr;//representa direcciones IP, con sus variantes Ipv4Addr e Ipv6Addr.
use std::str;

fn main() -> std::io::Result<()> {
    // Creado un socket UDP para cualquier IP local, puerto 8888
    // Equivalente a: let socket = UdpSocket::bind("0.0.0.0:8888")?;
    let socket = UdpSocket::bind(
        SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8888)))
        .expect("No puede utilizarse el puerto 8888");
    println!(
        "Abierto socket en IP: {}, Puerto: {}",
        socket.local_addr()?.ip(),
        socket.local_addr()?.port());

    // Recibe un datagrama UDP message y almacena su campo de datos en un buffer
    // El buffer debe ser suficientemente grande para contener los datos recibidos
    let mut buffer = [0; 10];
    let (len, client) = socket.recv_from(&mut buffer)?;

    // Convierte los datos a &str con str::from_utf8(&buffer)
    println!(
        "Recibido mensaje de longitud {len} del cliente {client}: {}",
        str::from_utf8(&buffer).expect("El mensaje no se puede convertir a string")
    );

    // Envía los datos recibidos al cliente
    socket.send_to(&buffer[..len], &client)?;

    Ok(())
}
