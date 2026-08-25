//trait: árbol de prefijos o árbol digital

//propósito principal: mejorar la búsqueda de los prefijos o de palabras

//en este caso, mejorar búsqueda de prefijos de direcciones IP, relativas a máscaras subred
//porque hay que considerar por qué interfaz subred enviar un determinado paquete (enrutarlo)

use std::net::Ipv4Addr;


#[derive(Debug)]
pub enum Trie {
    Empty,
    NonEmpty(Box<TrieNode>),
}
use Trie::*;


#[derive(Debug)]
pub struct TrieNode {
    iface: Option<String>,
    children: [Trie; 2], // hijo para bit 0 en la posición 0 del array children e
    // hijo para bit 1 en la posición 1 del array children
}


impl Trie {
    pub fn new() -> Trie { //el constructor new() crea el trie vacío
        Empty
    }


    // En el nodo raíz se podría guardar la ruta por defecto


    // Va recorriendo los Trienodes del árbol correspondientes a los bits de net_addr,
    // recorridos de izquierda a derecha, hasta que llega a la profundidad
    // subnet_mask, y entonces añade iface al TrieNode que está a esa profundidad


    // Si hay bits que no tienen nodo en el árbol, crea TrieNodes para ellos
    pub fn insert(
        &mut self, //referencia mutable a la estructura trie
        net_addr: Ipv4Addr, //dirección IP
        subnet_mask: usize, //máscara de subred
        iface: String, //interfaz de red
    ) -> Result<String, String> { //devuelve un result que puede tener en el caso positivo
        //una cadena de caracteres; y si fuese mal, el constructor Err con otra cadena de caracteres
        let &mut mut node; //declaración de la variable node, mutable
        //puntero inteligente que va a ir saltando de nodo a nodo de nuestro trie (ya que tiene estructura de árbol)
        //su tipo es ref mutable, porque su valor se va a ver modificado, además de que va a cambiar el sitio al que apunta

        // Si el árbol está vacío, creamos el TrieNode raíz.
        // En cualquier caso, después de esta sentencia match,
        //     node es una referencia al TrieNode raíz.
        match self {
            Empty => { //si el trie está vacío; *self es un puntero que atraviesa donde apunta
                *self = NonEmpty(Box::new(TrieNode { //modificamos el objeto a NonEmpty,
                    // y creamos un nuevo puntero inteligente que va a apuntar a un nuevo nodo de nuestro trie
                    iface: None, //que apunta a una interfaz que no va a tener
                    children: [Empty, Empty],// y a un array de hijos, que sus dos valores que son del tipo trie también
                    //el primer empty representa al bit 0, y el segundo al bit 1
                }));

                //esta n va a representa al puntero a puntero
                let NonEmpty(ref mut n) = *self else {
                    return Ok(Default::default()); //esta parte de ejecutaría en el caso de que no se pudiera haber
                    //reservado bien la memoria, o no hubiera suficiente
                };
                node = n; //node acaba apunta al mismo puntero que n, inicializo node
            }
            NonEmpty(ref mut n) => node = n, //para el resto de entradas después de la primera
        } //todo este bloque ha inicializado el puntero node apuntando al primer nodo de nuestra colección


        // bytes / octetos de la dirección IP que se inserta
        let net_addr_bytes = net_addr.octets(); // Método std::net::Ipv4Addr
        //devuelve un array con 4 octetos (posiciones) que son los bits de cada una de las partes
        //de la dirección IP

        // depth cuenta la profundidad. El nodo raíz está a profundidad 0
        // Cuando llegamos a profundidad depth, se añade en ese nodo la
        // iface
        let mut depth = 1usize; //inicializar la variable a 1
        //se inserta la interfaz de red a la profundidad que indique la subnet mask

        for byte in net_addr_bytes { //cada byte del array net_addr_bytes
            for i in (0..8).rev() { //bucle decremental
                // recorremos los 8 bits de izda a dcha (7,6,.. 0) -> MSB Big Endian
                // operación binaria shift right para obtener el valor de cada bit del byte
                let bit = (byte >> i) & 1; // valor del bit en la posición i del byte
                //variable para quedarnos con el bit en cuestión
                //desplazamiento de bits, tantas posiciones como valga la variable i -> (byte >> i)
                //aplicarle una máscara 1, bit and bit; lo que hace es limpiar todo el resto de bits
                //dejarlos todos a 0, y quedarnos con el bit que nos interesa

                // si no hay nodo hijo para bit, lo creamos
                if let Empty = node.children[bit as usize] { //meter en el nodo hijo asociado al bit que tengo,
                    // otro nodo más de nuestra estructura de datos
                    node.children[bit as usize] = NonEmpty(Box::new(TrieNode { //cambio el Empty del hijo correspondiente al bit
                        //por un NonEmpty con un puntero inteligente a un nuevo Nodo
                        iface: None, //con interfaz none
                        children: [Empty, Empty], //y con dos nodos hijos vacíos
                    }));
                }


                // visitamos el nodo hijo correspondiente a bit
                if let NonEmpty(ref mut children) = node.children[bit as usize] {
                    node = children; //dentro de la variable children se almacenan el puntero a ese nodo
                } //aquí hacemos que la variable node se desplace y baje a ese hijo en cuestión


                // si el bit está a profundidad depth == subnet_mask, añadimos la iface al nodo
                if depth == subnet_mask { //si la profundidad coincide con la máscara de red, hemos llegado al nodo al que nos interesa
                    node.iface = Some(iface.clone()); //con respecto a ese nodo modificamos su iface y meteríamos la iface que nos han
                    //pasado pero clonándola porque es un String


                    return Ok(format!(
                        "Añadida ruta para {}/{} -> {}",
                        net_addr, subnet_mask, iface
                    ));
                }


                depth += 1; //se va sumando la profundidad hasta llegar al nodo de la subnet mask
            } //e.g.: si la netmask de la IP es 24, los primeros 24 bits de la dirección IP representan a la subred
            //y a partir de ahí tendríamos direcciones disponibles para los dispositivos
        }
        Err(String::from(format!(
            "No se puede añadir la iface {} para la dirección {}/{}",
            iface, net_addr, subnet_mask
        )))
    }


    pub fn search(&self, dst_addr: &Ipv4Addr) -> Result<String, String> { //devuelvo un Result
        //hacemos un match de trie y guardamos lo que retorna en node (puntero inteligente)
        let mut node = match self { // a qué apuntas: trie vacio o no vacio??
            Trie::NonEmpty(ref n) => n, //en la variable n me almaceno una referencia no mut,
            //y eso mismo es lo que acbo devolviendo, lo que le asigno a node
            Trie::Empty => {
                return Err(format!("No hay ruta para la dirección {}", dst_addr));
            }
        };

        //como en la función anterior, creo una variable
        let dst_addr_bytes = dst_addr.octets(); // Para obtener los octetos de la dirección
        let mut best_iface: Option<String> = None; //¿cuál es la mejor interfaz por la que enrutar el paquete?

        //misma iteración que la función insert
        for byte in dst_addr_bytes {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 1; // Obtener el bit actual (de izquierda a derecha)
                //desplazamiento circular en el byte con N = a su indice; y enventanar

                // Intentar seguir al hijo correspondiente
                match &node.children[bit as usize] { //indexamos con el bit que tenemos entre manos
                    Trie::NonEmpty(ref child) => { //si el nodo no está vacío
                        node = child; // Avanzar al siguiente nodo
                        if let Some(ref iface) = node.iface { //guardar en iface una ref a la iface de ese nodo
                            best_iface = Some(iface.clone()); // Actualizar la mejor coincidencia encontrada
                        }
                    }
                    Trie::Empty => {
                        // No hay más nodos en este camino, terminamos la búsqueda
                        if let Some(iface) = best_iface {
                            return Ok(iface.clone());
                        } else {
                            return Err(format!("No hay ruta para la dirección {}", dst_addr));
                        }
                    }
                }
            }
        }


        // Si llegamos al final de la búsqueda, devolver la mejor coincidencia encontrada
        if let Some(iface) = best_iface {
            Ok(iface.clone())
        } else {
            Err(format!("No hay ruta para la dirección {}", dst_addr))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_insert_exists() {
        // Tabla de encaminamiento
        let mut trie = Trie::new(); //se encarga de crear la estructura de datos, el trie vacío


        // Entradas para la tabla de encaminamiento [ (ip, iface, mask) ]
        let entries = [
            (Ipv4Addr::new(192, 168, 0, 0), 24, String::from("eth0")),
            (Ipv4Addr::new(127, 0, 0, 0), 24, String::from("lo")),
            (Ipv4Addr::new(192, 168, 0, 32), 27, String::from("eth2")),
            (Ipv4Addr::new(192, 168, 0, 2), 32, String::from("eth1")),
        ];


        // Insertamos entries en trie
        for e in entries {
            let result = trie.insert(e.0.clone(), e.1, e.2.clone());
            match result {
                Ok(success) => {
                    println!("{}", success);
                }
                Err(_) => {}
            };
        }


        // Buscamos algunas direcciones
        let addresses = [
            Ipv4Addr::new(127, 0, 0, 1),
            Ipv4Addr::new(192, 167, 0, 1),
            Ipv4Addr::new(192, 168, 0, 33),
            Ipv4Addr::new(192, 168, 0, 2),
            Ipv4Addr::new(192, 168, 0, 127),
            Ipv4Addr::new(192, 168, 0, 1),
        ];


        let mut results = Vec::new();


        for ip_address in &addresses {
            let result = trie.search(ip_address);
            match result {
                Ok(iface) => {
                    results.push(iface);
                }
                Err(e) => {
                    results.push(e);
                }
            }
        }


        assert_eq!(
            results,
            vec![
                "lo",
                "No hay ruta para la dirección 192.167.0.1",
                "eth2",
                "eth1",
                "eth0",
                "eth0"
            ]
        );
    }
}


fn main() { }

