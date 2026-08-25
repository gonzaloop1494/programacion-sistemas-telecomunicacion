use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time::Duration; //Se utiliza std::thread::sleep() para esperar un tiempo
// y apreciar así mejor el funcionamiento del programa.

//el servidor en este ejercicio atiende a varios clientes a la vez, por la gestión de los hilos
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!(
        "\nSoy el servidor: {} y he recibido una conexión entrante de un cliente: {}.",
        stream.local_addr()?,
        stream.peer_addr()?
    );

    // Buffer para extraer el mensaje, suficientemente grande
    let mut buffer = [0; 100];

    // Lee datos del stream
    let len = stream.read(&mut buffer)?;

    // Convierte los datos a &str con str::from_utf8(&buffer)
    println!(
        "Recibido mensaje de longitud {len} del cliente: {}",
        str::from_utf8(&buffer).expect("El mensaje no se puede convertir a string")
    );

    println!("Esperando 5 segundos...");
    thread::sleep(Duration::from_secs(5));

    // Responde al cliente
    let respuesta = String::from("¡Hola, cliente!");
    stream.write_all(respuesta.as_bytes())?;
    println!("Enviada respuesta al cliente.");

    println!("Esperando 5 segundos para terminar la conexión...");
    thread::sleep(Duration::from_secs(5));
    println!("Cerrando la conexión.");

    Ok(())

    // Al cerrarse la función se cierra la conexión y se descarta el stream
}


fn main() -> std::io::Result<()> {

    // Crea un socket TCP para escuchar conexiones entrantes en el puerto 8888
    let listener = TcpListener::bind("0.0.0.0:8888")?;
    println!("Esperando conexiones...");

    // Para cada conexión entrante, lee el mensaje y responde
    for result_stream in listener.incoming() {
        // Obtiene el stream del io::Result<TcpStream> que devuelve el iterador
        let stream = result_stream?;

        thread::spawn(move || -> std::io::Result<()> {
            handle_connection(stream)?;
            Ok(())
        });

    }

    Ok(())
}
