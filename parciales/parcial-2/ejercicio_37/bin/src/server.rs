use std::env;
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::str;


const NUMBER_OF_ARGS: usize = 1 + 1;



fn main() -> std::io::Result<()> {
    //para tratar los argumentos que se reciben por la línea de comandos
    if env::args().len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin server server_port");
    }

    //me almaceno el puerto como un u16 en lugar de como un String
    let server_port = env::args().nth(1).unwrap().parse::<u16>().expect("El puerto debe ser un número válido");
                                //para convertir de string a u16: .parse::
                                //el .expect() genera un panic con el mensaje correspondiente
    //crear el socket o canal de comunicación para nuestro servidor
    let socket = UdpSocket::bind( //el bind hace que el proceso servidor se ate a una IP y a un puerto especifico
        SocketAddr::from((Ipv4Addr::UNSPECIFIED, server_port)))
        .expect(format!("No puede utilizarse el puerto {}", server_port).as_str());
    println!(
        "Abierto socket en IP: {}, Puerto: {}",
        socket.local_addr()?.ip(),
        socket.local_addr()?.port());


    loop {
        // se crea el buffer donde se va a depositar el mensaje que llegue de un cliente
        let mut buffer = [0; 1500]; //son cadenas de caracteres, pero llegan como bytes
        socket.recv_from(&mut buffer)?; //nos ponemos a recibir desde nuestro buffer (socket)
        //el .recv_from() es una instrucción bloqueante, quedo esperando a más bytes del cliente
        // (por funcionamiento del protocolo UDP)
        let message = str::from_utf8(&buffer).expect("El mensaje no se puede convertir a string");
        //proceso inverso para transformar de bytes a String
        match message.split_once('|') { //si no se encontrara la '|', entraría por el None
            // el message.split_once() devuelve una dupla
            Some((nick, text)) => {
                println!("{}: {}", nick, text);
            }
            None => {
                println!("Mensaje erróneo recibido: {}", message);
            }
        }
    }
}
