use std::collections::{HashMap, HashSet}; // Importa las estructuras de datos HashMap y HashSet
use std::hash::{DefaultHasher, Hash, Hasher}; // Importa herramientas para realizar hashing
use std::io::{prelude::*, BufReader}; // Importa funcionalidades de entrada/salida y BufReader para manejar streams
use std::net::{TcpListener, TcpStream}; // Importa estructuras para manejar conexiones TCP
use std::sync::{Arc, Mutex}; // Importa herramientas para manejo de concurrencia
use std::{env, str}; // Importa utilidades estándar como el manejo de argumentos y cadenas
use std::thread; // Importa soporte para trabajar con hilos

use rand::Rng; // Importa la biblioteca para generar números aleatorios

#[derive(Debug)] // Deriva el trait Debug para imprimir las partes de User
struct User {
    nickname: String, // Nombre de usuario del cliente
    group: Option<String>, // Grupo al que pertenece el usuario, si aplica
    stream: Option<TcpStream> // Stream de comunicación asociado al cliente
}

struct Storage {
    users: HashMap<String, User>, // Mapa que asocia hashes con información de usuarios
    groups: HashMap<String, HashSet<String>> // Mapa que asocia nombres de grupos con miembros (hashes de usuarios)
}

// Función para generar un hash único basado en el nickname y un número aleatorio
fn hash_nickname(nickname: &str) -> u64 {
    let random_number = rand::thread_rng().gen_range(0..=u32::MAX); // Genera un número aleatorio
    let input = format!("{}{}", nickname, random_number); // Combina el nickname y el número aleatorio
    let mut hasher = DefaultHasher::new(); // Crea una instancia de DefaultHasher
    input.hash(&mut hasher); // Realiza el hashing del input combinado
    hasher.finish() // Devuelve el hash generado
}

// Función que maneja la interacción con un cliente conectado
fn handle_client(
    mut stream: TcpStream, // Stream TCP asociado al cliente
    nicknames: Arc<Mutex<HashSet<String>>>, // Conjunto de nombres de usuario protegidos para concurrencia
    storage: Arc<Mutex<Storage>> // Estructura de almacenamiento compartida entre hilos
) -> std::io::Result<()> {
    println!(
        "\nSoy el servidor: {} y he recibido una conexión entrante de un cliente: {}.\n",
        stream.local_addr()?, // Dirección local del servidor
        stream.peer_addr()?, // Dirección del cliente conectado
    );

    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?); // BufReader para leer del stream del cliente
    let mut message = String::new(); // Buffer para almacenar los mensajes entrantes

    // Bucle principal para manejar mensajes del cliente
    while let Ok(bytes_read) = reader.read_line(&mut message) {
        if bytes_read == 0 { // Si no se leen bytes, se cierra la conexión
            break;
        }

        let words: Vec<&str> = message.trim().split_whitespace().collect(); // Divide el mensaje en palabras

        // Manejo del comando NEWUSER
        if words.len() == 2 && words[0] == ".newuser" {
            let nickname = words[1]; // Extrae el nickname del mensaje
            print!("Mensaje NEWUSER recibido con el nickname '{}': ", nickname);
            let mut unlocked_nicknames = nicknames.lock().unwrap(); // Bloquea el conjunto de nombres de usuario
            if unlocked_nicknames.contains(nickname) { // Verifica si el nickname ya existe
                println!("RECHAZADO");
                writeln!(stream, ".reject {}", nickname)?; // Responde con rechazo
            } else {
                println!("ACEPTADO");
                unlocked_nicknames.insert(nickname.to_string()); // Agrega el nickname al conjunto
                let hash = hash_nickname(nickname).to_string(); // Genera un hash para el usuario
                println!("Nombres de usuario: {:?}", unlocked_nicknames);
                let mut unlocked_storage = storage.lock().unwrap(); // Bloquea el almacenamiento compartido
                unlocked_storage.users.insert(hash.clone(), User {
                    nickname: nickname.to_string(),
                    group: None, // Inicialmente no pertenece a ningún grupo
                    stream: Some(stream.try_clone()?) // Clona el stream asociado al cliente
                });
                println!("Usuarios: {:?}", unlocked_storage.users);
                writeln!(stream, ".accept {} {}", nickname, hash.clone())?; // Responde con aceptación y el hash
            }
        }
        // Manejo de otros comandos después del hash
        else if words.len() >= 2 {
            let hash = words[0]; // El primer elemento es el hash del usuario
            {
                let mut unlocked_storage = storage.lock().unwrap(); // Bloquea el almacenamiento compartido
                if let Some(user) = unlocked_storage.users.get_mut(hash) {
                    user.stream = Some(stream.try_clone()?); // Actualiza el stream del usuario
                } else {
                    message.clear(); // Limpia el mensaje y continúa si el hash es inválido
                    continue;
                }
            };

            let command = words[1]; // El segundo elemento es el comando

            // Manejo de comandos simples
            if words.len() == 2 && command.starts_with(".") {
                match command {
                    ".list" => {
                        let unlocked_storage: std::sync::MutexGuard<'_, Storage> = storage.lock().unwrap(); // Bloquea el almacenamiento
                        let user: &User = unlocked_storage.users.get(hash).unwrap(); // Obtiene el usuario
                        let nickname: String = user.nickname.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?; // Clona el stream para responder
                            println!("Mensaje LIST recibido del usuario '{}'", nickname);

                            if unlocked_storage.groups.is_empty() {
                                writeln!(stream, ".info No existe ningún grupo actualmente")?; // Responde si no hay grupos
                            } else {
                                let group_names: Vec<String> = unlocked_storage.groups.keys().cloned().collect(); // Lista los nombres de grupos
                                writeln!(stream, ".info Lista de grupos: {}", group_names.join(" "))?; // Envía la lista de grupos
                            }
                        }
                    }
                    // Manejo del comando LEAVE
                    ".leave" => {
                        let mut unlocked_storage = storage.lock().unwrap(); // Bloquea el almacenamiento
                        let user = unlocked_storage.users.get(hash).unwrap(); // Obtiene el usuario
                        let nickname = user.nickname.clone();
                        let group = user.group.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?; // Clona el stream para responder
                            print!("Mensaje LEAVE recibido del usuario '{}': ", nickname);

                            match group {
                                Some(group_name) => {
                                    let members = unlocked_storage.groups.get(&group_name).unwrap(); // Obtiene los miembros del grupo
                                    for member_hash in members.iter() {
                                        if member_hash != hash {
                                            let member = unlocked_storage.users.get(member_hash).unwrap(); // Notifica a los miembros
                                            if let Some(ref original_member_stream) = member.stream {
                                                let mut member_stream = original_member_stream.try_clone()?;
                                                writeln!(member_stream, ".info {} ha salido del grupo", nickname)?;
                                            }
                                        }
                                    }
                                    let members = unlocked_storage.groups.get_mut(&group_name).unwrap(); // Remueve al usuario del grupo
                                    members.remove(hash);
                                    let user = unlocked_storage.users.get_mut(hash).unwrap();
                                    user.group = None; // Actualiza el estado del usuario
                                    println!("ha abandonado el grupo {} y se ha informado al resto de integrantes del mismo", group_name);
                                    writeln!(stream, ".info Te has salido del grupo '{}'", group_name)?; // Responde al usuario
                                }
                                None => {
                                    println!("no pertenece a ningún grupo");
                                    writeln!(stream, ".info Todavía no estás en ningún grupo")?; // Notifica que no pertenece a ningún grupo
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Mensaje DESCONOCIDO recibido"); // Comando desconocido
                    }
                }
            }
            // Manejo de comandos con argumentos (como CREATE y JOIN)
            else if words.len() == 3 && command.starts_with(".") {
                let group_name = words[2]; // Extrae el nombre del grupo
                match command {
                    ".create" => {
                        let mut unlocked_storage = storage.lock().unwrap(); // Bloquea el almacenamiento
                        let user = unlocked_storage.users.get(hash).unwrap(); // Obtiene el usuario
                        let nickname = user.nickname.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?; // Clona el stream para responder
                            print!("Mensaje CREATE recibido del usuario '{}' con el nombre de grupo '{}': ", nickname, group_name);

                            if unlocked_storage.groups.contains_key(group_name) {
                                println!("ya existe un grupo con el mismo nombre");
                                writeln!(stream, ".info No puedes crear el grupo '{}' porque ya existe uno con el mismo nombre", group_name)?; // Grupo ya existente
                            } else {
                                unlocked_storage.groups.insert(group_name.into(), HashSet::new()); // Crea el nuevo grupo
                                println!("el grupo ha sido creado");
                                writeln!(stream, ".info Has creado el grupo '{}'", group_name)?; // Notifica la creación
                            }
                        }
                    }
                    ".join" => {
                        let mut unlocked_storage = storage.lock().unwrap(); // Bloquea el almacenamiento
                        let user = unlocked_storage.users.get(hash).unwrap(); // Obtiene el usuario
                        let nickname = user.nickname.clone();
                        let group = user.group.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?; // Clona el stream para responder
                            println!("Mensaje JOIN recibido del usuario '{}' con el nombre de grupo '{}': ", nickname, group_name);

                            match group {
                                Some(user_group_name) => {
                                    println!("el usuario {} ya pertenece a otro grupo, concretamente al grupo '{}'.", nickname, user_group_name);
                                    writeln!(stream, ".info No puedes unirte al grupo '{}'. Primero tienes que salirte de tu grupo actual", group_name)?; // Ya está en otro grupo
                                }
                                None => {
                                    if let Some(members) = unlocked_storage.groups.get(group_name) {
                                        for member_hash in members.iter() {
                                            let member = unlocked_storage.users.get(member_hash).unwrap();
                                            if let Some(ref original_member_stream) = member.stream {
                                                let mut member_stream = original_member_stream.try_clone()?;
                                                writeln!(member_stream, ".info {} se ha unido al grupo", nickname)?; // Notifica a los miembros
                                            }
                                        }
                                        let members = unlocked_storage.groups.get_mut(group_name).unwrap(); // Añade al usuario al grupo
                                        members.insert(hash.into());
                                        let user = unlocked_storage.users.get_mut(hash).unwrap();
                                        user.group = Some(group_name.into()); // Actualiza el grupo del usuario
                                        println!("se ha unido a él el usuario {}", nickname);
                                        writeln!(stream, ".info Te has unido al grupo '{}'", group_name)?; // Notifica al usuario
                                    } else {
                                        println!("no existe un grupo con ese nombre");
                                        writeln!(stream, ".info No puedes unirte al grupo '{}' porque no existe", group_name)?; // Grupo inexistente
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Mensaje DESCONOCIDO recibido"); // Comando desconocido
                    }
                }
            } else {
                let (_, text) = message.split_once(' ').unwrap(); // Extrae el texto del mensaje

                let unlocked_storage = storage.lock().unwrap(); // Bloquea el almacenamiento
                let user = unlocked_storage.users.get(hash).unwrap(); // Obtiene el usuario
                let nickname = user.nickname.clone();
                let group = user.group.clone();
                if let Some(ref original_stream) = user.stream {
                    let mut stream = original_stream.try_clone()?; // Clona el stream para responder
                    println!("Mensaje TEXT recibido del usuario '{}' con el texto: {}", nickname, text.trim());

                    match group {
                        Some(group_name) => {
                            let members = unlocked_storage.groups.get(&group_name).unwrap(); // Obtiene los miembros del grupo
                            println!("Enviando el mensaje a todos los miembros del grupo '{}' excepto a '{}'", group_name, nickname);
                            for member_hash in members.iter() {
                                if member_hash != hash {
                                    let member = unlocked_storage.users.get(member_hash).unwrap(); // Notifica a cada miembro
                                    if let Some(ref original_member_stream) = member.stream {
                                        let mut member_stream = original_member_stream.try_clone()?;
                                        writeln!(member_stream, ".info {}: {}", nickname, text)?; // Envía el mensaje
                                    }
                                }
                            }
                        }
                        None => {
                            writeln!(stream, ".info Todavía no estás en ningún grupo")?; // Notifica que no pertenece a un grupo
                        }
                    }
                }
            }
        }
        println!(); // Línea vacía para separar las interacciones
        message.clear(); // Limpia el buffer del mensaje
    }

    Ok(()) // Retorna resultado exitoso
}

const NUMBER_OF_ARGS: usize = 1 + 1; // Define el número esperado de argumentos

fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS { // Verifica que el número de argumentos sea correcto
        panic!("cargo run --bin server puerto-servidor"); // Muestra un mensaje de error
    }
    let server_port = env::args().nth(1).unwrap(); // Obtiene el puerto del servidor

    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port))?; // Crea un listener en el puerto especificado
    println!("Esperando conexiones de clientes...");

    let nicknames: HashSet<String> = HashSet::new(); // Inicializa el conjunto de nombres de usuario
    let locked_nicknames: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(nicknames)); // Protege el conjunto con Arc y Mutex

    let storage = Storage {
        users: HashMap::new(), // Inicializa el mapa de usuarios
        groups: HashMap::new() // Inicializa el mapa de grupos
    };
    let locked_storage: Arc<Mutex<Storage>> = Arc::new(Mutex::new(storage)); // Protege el almacenamiento con Arc y Mutex

    for result_stream in listener.incoming() { // Itera sobre las conexiones entrantes
        let stream = result_stream?; // Obtiene el stream TCP
        let arc_nicknames = Arc::clone(&locked_nicknames); // Clona el Arc de nombres de usuario
        let arc_storage = Arc::clone(&locked_storage); // Clona el Arc de almacenamiento

        thread::spawn(move || -> std::io::Result<()> { // Crea un nuevo hilo para manejar al cliente
            handle_client(stream, arc_nicknames, arc_storage)?; // Llama a la función handle_client
            Ok(()) // Finaliza el hilo exitosamente
        });
    }

    Ok(()) // Retorna resultado exitoso
}

