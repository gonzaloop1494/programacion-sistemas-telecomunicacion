use std::env;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;


use colored::Colorize;


const NUMBER_OF_ARGS: usize = 1 + 2;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin client ip-servidor puerto-servidor");
    }
    let server_ip = args[1].clone();
    let server_port = args[2].clone();


    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;

    println!(
        "Soy un cliente: {} y he establecido una conexión con el servidor: {}.",
        stream.local_addr()?,
        stream.peer_addr()?
    );


    let mut reader = BufReader::new(stream.try_clone()?);

    for line_result in std::io::stdin().lines() {
        let line: String = line_result?;
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        if let Some(&command) = words.get(0) {
            match command {
                ".numbers" => {
                    writeln!(stream, ".numbers {}", &words[1..].join(" "))?;
                    let mut message = String::new();
                    let bytes_read = reader.read_line(&mut message).unwrap();
                    if bytes_read == 0 {
                        println!("El servidor se ha desconectado");
                        break;
                    }
                    println!("Resultado de la suma: {}", message);
                }
                ".strings" => {
                    writeln!(stream, ".strings {}", &words[1..].join(" "))?;
                    let mut message = String::new();
                    let bytes_read = reader.read_line(&mut message).unwrap();
                    if bytes_read == 0 {
                        println!("El servidor se ha desconectado");
                        break;
                    }
                    println!("Resultado de la concatenación al revés: {}", message);
                }
                ".quit" => {
                    writeln!(stream, ".quit")?;
                    break;
                }
                _ => {
                    println!("{}", format!("Comando '{}' desconocido", command).red());
                }
            }
            println!();
        } else {
            println!("{}", format!("No se ha introducido ningún comando\n").red());
        }
    }

    Ok(())
}
