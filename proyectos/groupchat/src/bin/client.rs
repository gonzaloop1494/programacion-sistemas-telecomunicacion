use std::fs::File; // Módulo para manejo de ficheros
use std::{env, thread}; // //módulo para pasar de forma sencilla datos a un programa
// Módulos para entorno y manejo de hilos
use std::io::{BufReader, BufWriter, Write, BufRead};//módulos para entrada/salida
use std::net::TcpStream;//módulo para conexiones TCP
use colored::*;


//constante para validar que se reciban los argumentos necesarios al iniciar el programa
const EXPECTED_ARGUMENT_COUNT: usize = 4; // Nombre del programa + 3 argumentos
const CREDENTIALS_FILE: &str = "client_credentials.txt"; //almacena el nombre del archivo donde guardamos las credenciales
//del usuario

fn main() -> std::io::Result<()> {
    // Validar número de argumentos
    if env::args().len() != EXPECTED_ARGUMENT_COUNT { //Verificamos que el usuario proporcione la IP-server, el server-port y un nickname
        panic!("Uso: cargo run --bin client <ip-servidor> <puerto-servidor> <nickname>");
    } //si no lo hace, el programa muestra un mensaje de error y se detiene

    // Leer argumentos de línea de comandos
    let server_ip = env::args().nth(1).expect("Falta la IP del servidor");
    let server_port = env::args().nth(2).expect("Falta el puerto del servidor");
    let client_nickname = env::args().nth(3).expect("Falta el nickname del cliente");

    // Establecer conexión con el servidor
    //Usando la IP y el puerto proporcionados
    let mut connection = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;
    println!(
        "\nCliente conectado al servidor desde {} hacia {}.\n",
        connection.local_addr()?,
        connection.peer_addr()?
    );

    let mut client_hash = String::new(); //constructor que crea variable mut para guardar la hash del cliente
    let mut is_new_user = true; //variable mut booleana que, en caso de no ser newuser client, es true

    // Intentar cargar credenciales existentes
    match File::open(CREDENTIALS_FILE) {
        Ok(file) => { //si lo encuentra, intenta leer el nickname y el hash guardados
            let mut file_reader = BufReader::new(file); //leer lineas de un fichero de texto
            let mut credentials = String::new();//donde se van almacenando los datos del fichero
            if file_reader.read_line(&mut credentials)? > 0 { //si el fichero contiene bytes (no está vacío)
                if let Some((stored_nickname, stored_hash)) = credentials.trim_end().split_once(' ') {
                    if client_nickname == stored_nickname { //compruebo si coinciden el nickname del nuevo client con uno que ya está en credenciales
                        client_hash = stored_hash.to_string(); //obtengo su hash asociada
                        is_new_user = false; //NO HAY QUE ENVIAR MENSAJE NEWUSER
                    }
                }
            }
        }
        Err(_) => { //el archivo de credenciales de clientes está vacío
            println!("No se encontraron credenciales previas.");
        }
    }

    // Enviar mensaje NEWUSER si es un usuario nuevo
    if is_new_user { //mensaje newuser (su nickname no estaba en las credenciales)
        writeln!(connection, ".newuser {}", client_nickname)?; //se envía al servidor el mensaje NEWUSER
        //por el TCPStream

        let mut server_response = String::new(); //var mut donde voy a almacenar la respuesta del servidor
        let mut response_reader = BufReader::new(connection.try_clone()?); //permite leer la respuesta del servidor
        if response_reader.read_line(&mut server_response)? == 0 {
            panic!("Respuesta vacía del servidor al mensaje NEWUSER");
        }

        let response_parts: Vec<&str> = server_response.trim().split_whitespace().collect(); //separo la respuesta del servidor
        match response_parts.as_slice() {
            [".accept", nickname, hash] => { //si es accept
                client_hash = hash.to_string(); //me guardo la hash que me aporta el servidor para posteriores mensajes
                println!("Conectado al servidor con el nickname '{}'.\n", nickname);

                let mut credentials_file = BufWriter::new(File::create(CREDENTIALS_FILE)?);
                writeln!(credentials_file, "{} {}", nickname, client_hash)?; //el cliente guarda en el fichero sus credenciales
                //nickname, y su hash asociada
                credentials_file.flush()?; //asegurarse de que los datos pendientes en un buffer
                // sean escritos inmediatamente en su destino subyacente
            }
            [".reject", nickname] => { //si la respuesta es reject
                println!("El nickname '{}' ya está en uso. Intenta con otro.", nickname);
                return Ok(());
            }
            _ => panic!("Formato desconocido en la respuesta del servidor: {}", server_response),
        }
    }

    // Crear hilo para escuchar mensajes del servidor
    let server_listener = connection.try_clone()?;
    thread::spawn(move || -> std::io::Result<()> { //para manejar los mensajes entrantes del servidor, creamos un hilo separado
        let server_reader = BufReader::new(server_listener); //lee los mensajes del servidor
        for line in server_reader.lines() { //almaceno en línea los mensajes que llegan al buffer procedentes del server
            match line {
                Ok(message) => {
                    if let Some((".info", info)) = message.split_once(' ') {
                        println!("{}", info.blue());
                    }
                }
                Err(e) => println!("Error al leer mensaje del servidor: {}", e), //en caso de un error en la lectura del mensaje
            }
        }
        Ok(())
    });

    // Leer comandos desde la entrada estándar
    let stdin_reader = std::io::stdin(); //para leer de la entrada estandar
    for user_input in stdin_reader.lock().lines() { //guardo en user_input la entrada introducida por el usuario
        let input = user_input?; //guardo aqui la entrada
        let input_parts: Vec<&str> = input.trim().split_whitespace().collect(); //lo recolecto en un vector, cuyos elementos
        //son cada argumento en la linea de comandos

        if let Some(&command) = input_parts.get(0) { //acceder al comando
            match command {
                ".list" => {
                    writeln!(connection, "{} .list", client_hash)?;
                }
                ".create" => {
                    if let Some(&group_name) = input_parts.get(1) {
                        writeln!(connection, "{} .create {}", client_hash, group_name)?;
                    } else {
                        println!("Especifica un nombre de grupo para crear.");
                    }
                }
                ".join" => {
                    if let Some(&group_name) = input_parts.get(1) {
                        writeln!(connection, "{} .join {}", client_hash, group_name)?;
                    } else {
                        println!("Especifica un nombre de grupo para unirte.");
                    }
                }
                ".leave" => {
                    writeln!(connection, "{} .leave", client_hash)?;
                }
                ".quit" => {
                    writeln!(connection, "{} .quit", client_hash)?;
                    break;
                }
                _ => {
                    if command.starts_with('.') {
                        println!("{}", "Comando desconocido.".red());
                    } else {
                        writeln!(connection, "{} {}", client_hash, input)?;
                    }
                }
            }
        }
    }

    Ok(())
}