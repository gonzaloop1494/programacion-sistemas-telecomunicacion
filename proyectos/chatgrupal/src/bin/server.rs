use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{env, str};
use std::thread;


use rand::Rng;


#[derive(Debug)]
struct User {
    nickname: String,
    group: Option<String>,
    stream: Option<TcpStream>
}


struct Storage {
    users: HashMap<String, User>,
    groups: HashMap<String, HashSet<String>>
}


fn hash_nickname(nickname: &str) -> u64 {
    let random_number= rand::thread_rng().gen_range(0..=u32::MAX);
    let input = format!("{}{}", nickname, random_number);
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn handle_client(
    mut stream: TcpStream,
    nicknames: Arc<Mutex<HashSet<String>>>,
    storage: Arc<Mutex<Storage>>) -> std::io::Result<()> {
    println!(
        "\nSoy el servidor: {} y he recibido una conexión entrante de un cliente: {}.\n",
        stream.local_addr()?,
        stream.peer_addr()?,
    );
    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);

    let mut message = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut message) {
        if bytes_read == 0 {
            break;
        }


        let words: Vec<&str> = message.trim().split_whitespace().collect();
        if words.len() == 2 && words[0] == ".newuser" {
            let nickname = words[1];
            print!("Mensaje NEWUSER recibido con el nickname '{}': ", nickname);
            let mut unlocked_nicknames = nicknames.lock().unwrap();
            if unlocked_nicknames.contains(nickname) {
                println!("RECHAZADO");
                writeln!(stream, ".reject {}", nickname)?;
            } else {
                println!("ACEPTADO");
                unlocked_nicknames.insert(nickname.to_string());
                let hash = hash_nickname(nickname).to_string();
                println!("Nombres de usuario: {:?}", unlocked_nicknames);
                let mut unlocked_storage = storage.lock().unwrap();
                unlocked_storage.users.insert(hash.clone(), User {nickname: nickname.to_string(), group: None, stream: Some(stream.try_clone()?)});
                println!("Usuarios: {:?}", unlocked_storage.users);
                writeln!(stream, ".accept {} {}", nickname, hash.clone())?;
            }
        } else if words.len() >= 2 {
            let hash = words[0];
            {
                let mut unlocked_storage = storage.lock().unwrap();
                if let Some(user) = unlocked_storage.users.get_mut(hash) {
                    user.stream = Some(stream.try_clone()?);
                } else {
                    message.clear();
                    continue;
                }
            };
            let command = words[1];
            if words.len() == 2 && command.starts_with(".") {
                match command {
                    ".list" => {
                        let unlocked_storage: std::sync::MutexGuard<'_, Storage> = storage.lock().unwrap();
                        let user: &User = unlocked_storage.users.get(hash).unwrap();
                        let nickname: String = user.nickname.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?;
                            println!("Mensaje LIST recibido del usuario '{}'", nickname);


                            if unlocked_storage.groups.is_empty() {
                                writeln!(stream, ".info No existe ningún grupo actualmente")?;
                            } else {
                                let group_names: Vec<String> = unlocked_storage.groups.keys().cloned().collect();
                                writeln!(stream, ".info Lista de grupos: {}", group_names.join(" "))?;
                            }
                        }
                    }
                    ".leave" => {
                        let mut unlocked_storage = storage.lock().unwrap();
                        let user = unlocked_storage.users.get(hash).unwrap();
                        let nickname = user.nickname.clone();
                        let group = user.group.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?;
                            print!("Mensaje LEAVE recibido del usuario '{}': ", nickname);


                            match group {
                                Some(group_name) => {
                                    let members = unlocked_storage.groups.get(&group_name).unwrap();
                                    for member_hash in members.iter() {
                                        if member_hash != hash {
                                            let member = unlocked_storage.users.get(member_hash).unwrap();
                                            if let Some(ref original_member_stream) = member.stream {
                                                let mut member_stream = original_member_stream.try_clone()?;
                                                writeln!(member_stream, ".info {} ha salido del grupo", nickname)?;
                                            }
                                        }
                                    }
                                    let members = unlocked_storage.groups.get_mut(&group_name).unwrap();
                                    members.remove(hash);
                                    let user = unlocked_storage.users.get_mut(hash).unwrap();
                                    user.group = None;
                                    println!("ha abandonado el grupo {} y se ha informado al resto de integrantes del mismo", group_name);
                                    writeln!(stream, ".info Te has salido del grupo '{}'", group_name)?;
                                }
                                None => {
                                    println!("no pertenece a ningún grupo");
                                    writeln!(stream, ".info Todavía no estás en ningún grupo")?;
                                }
                            }
                        }
                    }
                    ".quit" => {
                        let mut unlocked_storage = storage.lock().unwrap();
                        let user = unlocked_storage.users.get(hash).unwrap();
                        let nickname = user.nickname.clone();
                        let group = user.group.clone();
                        println!("Mensaje QUIT recibido del usuario '{}'", nickname);

                        match group {
                            Some(group_name) => {
                                let members = unlocked_storage.groups.get(&group_name).unwrap();
                                for member_hash in members.iter() {
                                    if member_hash != hash {
                                        let member = unlocked_storage.users.get(member_hash).unwrap();
                                        if let Some(ref original_member_stream) = member.stream {
                                            let mut member_stream = original_member_stream.try_clone()?;
                                            writeln!(member_stream, ".info {} se ha desconectado del chat", nickname)?;
                                        }
                                    }
                                }
                                let members = unlocked_storage.groups.get_mut(&group_name).unwrap();
                                members.remove(hash);
                                let user = unlocked_storage.users.get_mut(hash).unwrap();
                                user.group = None;
                                println!("ha abandonado el grupo {}, avisando al resto de integrantes, y se ha desconectado del chat", group_name);
                            }
                            None => {}
                        }

                        let user = unlocked_storage.users.get_mut(hash).unwrap();
                        user.stream = None;
                    }
                    _ => {
                        println!("Mensaje DESCONOCIDO recibido");
                    }
                }
            } else if words.len() == 3 && command.starts_with(".") {
                let group_name = words[2];
                match command {
                    ".create" => {
                        let mut unlocked_storage = storage.lock().unwrap();
                        let user = unlocked_storage.users.get(hash).unwrap();
                        let nickname = user.nickname.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?;
                            print!("Mensaje CREATE recibido del usuario '{}' con el nombre de grupo '{}': ", nickname, group_name);

                            if unlocked_storage.groups.contains_key(group_name) {
                                println!("ya existe un grupo con el mismo nombre");
                                writeln!(stream, ".info No puedes crear el grupo '{}' porque ya existe uno con el mismo nombre", group_name)?;
                            } else {
                                unlocked_storage.groups.insert(group_name.into(), HashSet::new());
                                println!("el grupo ha sido creado");
                                writeln!(stream, ".info Has creado el grupo '{}'", group_name)?;
                            }
                        }
                    }
                    ".join" => {
                        let mut unlocked_storage = storage.lock().unwrap();
                        let user = unlocked_storage.users.get(hash).unwrap();
                        let nickname = user.nickname.clone();
                        let group = user.group.clone();
                        if let Some(ref original_stream) = user.stream {
                            let mut stream = original_stream.try_clone()?;
                            println!("Mensaje JOIN recibido del usuario '{}' con el nombre de grupo '{}': ", nickname, group_name);


                            match group {
                                Some(user_group_name) => {
                                    println!("el usuario {} ya pertenece a otro grupo, concretamente al grupo '{}'.", nickname, user_group_name);
                                    writeln!(stream, ".info No puedes unirte al grupo '{}'. Primero tienes que salirte de tu grupo actual", group_name)?;
                                }
                                None => {
                                    if let Some(members) = unlocked_storage.groups.get(group_name) {
                                        for member_hash in members.iter() {
                                            let member = unlocked_storage.users.get(member_hash).unwrap();
                                            if let Some(ref original_member_stream) = member.stream {
                                                let mut member_stream = original_member_stream.try_clone()?;
                                                writeln!(member_stream, ".info {} se ha unido al grupo", nickname)?;
                                            }
                                        }
                                        let members = unlocked_storage.groups.get_mut(group_name).unwrap();
                                        members.insert(hash.into());
                                        let user = unlocked_storage.users.get_mut(hash).unwrap();
                                        user.group = Some(group_name.into());
                                        println!("se ha unido a él el usuario {}", nickname);
                                        writeln!(stream, ".info Te has unido al grupo '{}'", group_name)?;
                                    } else {
                                        println!("no existe un grupo con ese nombre");
                                        writeln!(stream, ".info No puedes unirte al grupo '{}' porque no existe", group_name)?;
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Mensaje DESCONOCIDO recibido");
                    }
                }
            } else {
                let (_, text) = message.split_once(' ').unwrap();


                let unlocked_storage = storage.lock().unwrap();
                let user = unlocked_storage.users.get(hash).unwrap();
                let nickname = user.nickname.clone();
                let group = user.group.clone();
                if let Some(ref original_stream) = user.stream {
                    let mut stream = original_stream.try_clone()?;
                    println!("Mensaje TEXT recibido del usuario '{}' con el texto: {}", nickname, text.trim());

                    match group {
                        Some(group_name) => {
                            let members = unlocked_storage.groups.get(&group_name).unwrap();
                            println!("Enviando el mensaje a todos los miembros del grupo '{}' excepto a '{}'", group_name, nickname);
                            for member_hash in members.iter() {
                                if member_hash != hash {
                                    let member = unlocked_storage.users.get(member_hash).unwrap();
                                    if let Some(ref original_member_stream) = member.stream {
                                        let mut member_stream = original_member_stream.try_clone()?;
                                        writeln!(member_stream, ".info {}: {}", nickname, text)?;
                                    }
                                }
                            }
                        }
                        None => {
                            writeln!(stream, ".info Todavía no estás en ningún grupo")?;
                        }
                    }
                }
            }
        }
        println!();
        message.clear();
    }

    Ok(())
}


const NUMBER_OF_ARGS: usize = 1 + 1;

fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin server puerto-servidor");
    }
    let server_port = env::args().nth(1).unwrap(); // Otra alternativa es transformar el String a un u16

    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port))?;
    println!("Esperando conexiones de clientes...");


    let nicknames: HashSet<String> = HashSet::new();
    let locked_nicknames: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(nicknames));


    let storage = Storage {
        users: HashMap::new(),
        groups: HashMap::new()
    };
    let locked_storage: Arc<Mutex<Storage>> = Arc::new(Mutex::new(storage));


    for result_stream in listener.incoming() {
        let stream = result_stream?;
        let arc_nicknames = Arc::clone(&locked_nicknames);
        let arc_storage = Arc::clone(&locked_storage);


        thread::spawn(move || -> std::io::Result<()> {
            handle_client(stream, arc_nicknames, arc_storage)?;
            Ok(())
        });
    }

    Ok(())
}
