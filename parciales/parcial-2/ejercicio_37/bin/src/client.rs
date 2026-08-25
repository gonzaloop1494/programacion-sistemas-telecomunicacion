use std::env;
use std::io;
use std::io::Write;
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;


const NUMBER_OF_ARGS: usize = 1 + 3;

fn main() -> std::io::Result<()> {
    //para tratar los argumentos que se reciben por la línea de comandos
    if env::args().len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin client nick server_ip server_port");
    }


    let nick = env::args().nth(1).unwrap();
    let server_ip = env::args().nth(2).unwrap();
    let server_port = env::args().nth(3).unwrap(); // El puerto aquí lo trato como un String

    //enlazo al cliente mediante un bind, pero con el puerto 0, que identifica a cualquier puerto que esté libre
    //del 1024 incluido en adelante (de forma aleatoria)
    //el servidor, sin embargo, debe de tener un puerto conocido
    let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))?;
    println!(
        "Abierto socket en IP: {}, Puerto: {}",
        socket.local_addr()?.ip(),
        socket.local_addr()?.port());
    // el cliente almacena aquí la dirección del servidor (con su IP, y su puerto)
    let server_address = format!("{}:{}", server_ip, server_port);

    //construimos desde 0 un String que va a tener el mensaje a enviar del cliente al servidor
    let mut text = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut text) { //espera a que el usuario introduzca
        // un mensaje por teclado, si va bien, se almacena en el String Text
        if text.trim() == ".fin" { //cuando detecta el comando .fin, termina toda interacción con el servidor
            break;
        }

        //si el comando no es .fin, se envía un mensaje al servidor con el nick del cliente y el texto introducido
        let message = format!("{}|{}", nick, text.trim());
        socket.send_to(message.as_bytes(), server_address.clone())?; //fromato para mandar mensaje cliente/servidor en UDP
           //message.as_bytes() -> para enviar el mensaje en bytes
                        //server.adress.clone() -> dirección del server, hay que clonar la address

        text.clear(); //para que no se solapen unos mensajes con otros
    }


    Ok(())
}
