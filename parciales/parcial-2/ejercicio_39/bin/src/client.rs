use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::thread;
use std::time::Duration; //Se utiliza std::thread::sleep() para esperar un tiempo
// y apreciar así mejor el funcionamiento del programa.

fn main() -> std::io::Result<()> {

    // Crea un socket TCP para y abre la conexión con el servidor
    let mut stream = TcpStream::connect("127.0.0.1:8888")?;

    println!(
        "Soy un cliente: {} y he establecido una conexión con el servidor: {}.",
        stream.local_addr()?,
        stream.peer_addr()?
    );

    println!("Esperando 5 segundos...");
    thread::sleep(Duration::from_secs(5));

    // String a enviar
    let m = String::from("¡Hola, servidor!");

    // Convierte el String a un array de bytes y lo envía a través del stream
    stream.write_all(m.as_bytes())?;
    println!("Enviado mensaje al servidor");

    // Buffer para extraer el mensaje de respuesta, suficientemente grande
    let mut buffer = [0; 1500];

    // Recibe la respuesta del servidor leyendo del stream
    let len = stream.read(&mut buffer)?;

    // Convierte los datos recibidos a &str con std::str::from_utf8()
    println!(
        "Recibido mensaje ({len} bytes) del servidor: {}",
        str::from_utf8(&buffer[..len])
            .expect("El mensaje no se puede convertir a string")
    );

    Ok(())

    // Al cerrarse el ámbito se descarta el stream y se cierra la conexión
}
