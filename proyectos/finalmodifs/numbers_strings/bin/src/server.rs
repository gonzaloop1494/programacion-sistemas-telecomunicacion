use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::env;

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!(
        "\nSoy el servidor: {} y he recibido una conexión entrante de un cliente: {}.",
        stream.local_addr()?,
        stream.peer_addr()?
    );

    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);
    let mut message = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut message) {
        if bytes_read == 0 {
            break;
        }
        println!("Mensaje recibido: {}", message);


        let words: Vec<&str> = message.trim().split_whitespace().collect();
        let header = words[0];
        let tail = &words[1..].join(" ");
        match header {
            ".numbers" => {
                let mut result: u32 = 0;
                let elements: Vec<&str> = tail.trim().split_whitespace().collect();
                for str_element in elements {
                    let element = str_element.parse::<u32>().unwrap();
                    result += element;
                }
                writeln!(stream, "{}", result)?;
            }
            ".strings" => {
                let mut result = String::new();
                let elements: Vec<&str> = tail.trim().split_whitespace().collect();
                for index in (0..elements.len()).rev() {
                    let element = elements[index];
                    result.push_str(element);
                }
                writeln!(stream, "{}", result)?;
            }
            ".quit" => {
                println!("El cliente '{}' se ha desconectado", stream.peer_addr()?);
            }
            _ => {
                println!("Mensaje DESCONOCIDO recibido");
            }
        }
        message.clear();
    }

    Ok(())
}

const NUMBER_OF_ARGS: usize = 1 + 1;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin server puerto-servidor");
    }
    let port = args[1].clone();


    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    println!("Esperando conexiones...");

    for result_stream in listener.incoming() {
        let stream = result_stream?;

        thread::spawn(move || -> std::io::Result<()> {
            handle_connection(stream)?;
            Ok(())
        });
    }

    Ok(())
}
