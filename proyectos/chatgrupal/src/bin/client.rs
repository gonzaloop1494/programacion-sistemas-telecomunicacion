use std::fs::File; //módulo para usar ficheros
use std::{env, thread}; //módulo para pasar de forma sencilla datos a un programa
use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpStream;


use colored::*;


const NUMBER_OF_ARGS: usize = 1 + 3;
const CREDENTIALS_FILE_NAME: &str = "credentials.txt";


fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin client ip-servidor puerto-servidor nickname");
    }
    let server_ip = env::args().nth(1).unwrap();
    let server_port = env::args().nth(2).unwrap(); // Otra alternativa es transformar el String a un u16
    let nickname = env::args().nth(3).unwrap();

    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;

    println!(
        "\nSoy un cliente: {} y he establecido una conexión con el servidor: {}.\n",
        stream.local_addr()?,
        stream.peer_addr()?
    );


    let mut hash = String::new();

    let mut must_send_newuser_message = false;
    // NEWUSER
    match File::open(CREDENTIALS_FILE_NAME) {
        Ok(credentials_file) => {
            let mut reader = std::io::BufReader::new(credentials_file);
            let mut line = String::new();


            // Leer una línea
            match reader.read_line(&mut line) {
                Ok(0) => {
                    println!("El archivo está vacío");
                    must_send_newuser_message = true;
                }
                Ok(_) => {
                    if let Some((file_nickname, file_hash)) = line.trim_end().split_once(' ') {
                        if nickname == file_nickname {
                            hash = file_hash.into();
                        } else {
                            must_send_newuser_message = true;
                        }
                    }
                }
                Err(e) =>  {
                    println!("Error al leer la línea: {}", e);
                    must_send_newuser_message = true;
                }
            }
        }
        Err(_) => {
            must_send_newuser_message = true;
        }
    }

    let mut must_interact_with_server = true;
    if must_send_newuser_message {
        writeln!(stream, ".newuser {}", nickname)?;


        // Response
        let mut message = String::new();
        let mut reader = BufReader::new(stream.try_clone()?); // Podríamos haber omitido el BufReader
        let bytes_read = reader.read_line(&mut message)?;
        if bytes_read == 0 {
            panic!("Se han recibido 0 bytes como respuesta del servidor al mensaje NEWUSER");
        }
        let words: Vec<&str> = message.trim().split_whitespace().collect();
        if words.len() == 3 && words[0] == ".accept" {
            let nickname = words[1];
            hash  = words[2].into();
            println!("Conectado al servidor con el nickname '{}'\n", nickname);


            let credentials_file = File::create(CREDENTIALS_FILE_NAME)?;
            let mut writer = BufWriter::new(credentials_file);
            writeln!(writer, "{} {}", nickname, hash)?;
            writer.flush()?;
        } else if words.len() == 2 && words[0] == ".reject" {
            let nickname = words[1];
            println!("No se puede hacer utilizar el nickname '{}' porque ya está en uso en el GroupChat", nickname);
            must_interact_with_server = false;
        }
    }


    if must_interact_with_server { //Se le puede asociar un std::io::BufReader o un std::io:BufWriter,
        // pero eso transfiere la propiedad del stream.
        // Si se quiere seguir usando es necesario usar el método try_clone() que devuelve un Result<TcpStream> clon.
        let reader = BufReader::new(stream.try_clone()?);
        thread::spawn(|| -> std::io::Result<()> {
            for line_result in reader.lines() {
                let message = line_result?;
                if let Some((".info", text)) = message.split_once(' ') {
                    println!("{}", format!("{}\n", text).blue());
                }
            }
            Ok(())
        });


        for line_result in std::io::stdin().lines() {
            let line = line_result?;
            let words: Vec<&str> = line.trim().split_whitespace().collect();
            if let Some(&command) = words.get(0) {
                match command {
                    ".list" => {
                        writeln!(stream, "{} .list", hash)?;
                    }
                    ".create" => {
                        let group = words[1];
                        writeln!(stream, "{} .create {}", hash, group)?;
                    }
                    ".join" => {
                        let group = words[1];
                        writeln!(stream, "{} .join {}", hash, group)?;
                    }
                    ".leave" => {
                        writeln!(stream, "{} .leave", hash)?;
                    }
                    ".quit" => {
                        writeln!(stream, "{} .quit", hash)?;
                        break;
                    }
                    _ => {
                        if command.starts_with('.') {
                            println!("{}", format!("Comando desconocido").red());
                        } else {
                            let text = line;
                            writeln!(stream, "{} {}", hash, text)?;
                        }
                    }
                }
                println!();
            }
        }
    }

    Ok(())
}
