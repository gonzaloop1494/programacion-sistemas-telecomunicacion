mod maincopy;

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
    pub fn new() -> Trie {
        Empty
    }


    // En el nodo raíz se podría guardar la ruta por defecto


    // Va recorriendo los Trienodes del árbol correspondientes a los bits de net_addr,
    // recorridos de izquierda a derecha, hasta que llega a la profundidad
    // subnet_mask, y entonces añade iface al TrieNode que está a esa profundidad


    // Si hay bits que no tienen nodo en el árbol, crea TrieNodes para ellos
    pub fn insert(
        &mut self,
        net_addr: Ipv4Addr,
        subnet_mask: usize,
        iface: String,
    ) -> Result<String, String> {
        let &mut mut node;


        // Si el árbol está vacío, creamos el TrieNode raíz.
        // En cualquier caso, después de esta sentencia match,
        //     node es una referencia al TrieNode raíz.
        match self {
            Empty => {
                *self = NonEmpty(Box::new(TrieNode {
                    iface: None,
                    children: [Empty, Empty],
                }));


                let NonEmpty(ref mut n) = *self else {
                    return Ok(Default::default());
                };
                node = n;
            }
            NonEmpty(ref mut n) => node = n,
        }


        // bytes / octetos de la dirección IP que se inserta
        let net_addr_bytes = net_addr.octets(); // Método std::net::Ipv4Addr


        // depth cuenta la profundidad. El nodo raíz está a profundidad 0
        // Cuando llegamos a profundidad depth, se añade en ese nodo la
        // iface
        let mut depth = 1usize;


        for byte in net_addr_bytes {
            for i in (0..8).rev() {
                // recorremos los 8 bits de izda a dcha (7,6,.. 0)
                // operación binaria shift right para obtener el valor de cada bit del byte
                let bit = (byte >> i) & 1; // valor del bit en la posición i del byte


                // si no hay nodo hijo para bit, lo creamos
                if let Empty = node.children[bit as usize] {
                    node.children[bit as usize] = NonEmpty(Box::new(TrieNode {
                        iface: None,
                        children: [Empty, Empty],
                    }));
                }


                // visitamos el nodo hijo correspondiente a bit
                if let NonEmpty(ref mut children) = node.children[bit as usize] {
                    node = children;
                }


                // si el bit está a profundidad depth == subnet_mask, añadimos la iface al nodo
                if depth == subnet_mask {
                    node.iface = Some(iface.clone());


                    return Ok(format!(
                        "Añadida ruta para {}/{} -> {}",
                        net_addr, subnet_mask, iface
                    ));
                }


                depth += 1;
            }
        }
        Err(String::from(format!(
            "No se puede añadir la iface {} para la dirección {}/{}",
            iface, net_addr, subnet_mask
        )))
    }


    pub fn search(&self, dst_addr: &Ipv4Addr) -> Result<String, String> {
        let mut node = match self {
            Trie::NonEmpty(ref n) => n,
            Trie::Empty => {
                return Err(format!("No hay ruta para la dirección {}", dst_addr));
            }
        };


        let dst_addr_bytes = dst_addr.octets(); // Obtener los octetos de la dirección
        let mut best_iface: Option<String> = None;


        for byte in dst_addr_bytes {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 1; // Obtener el bit actual (de izquierda a derecha)


                // Intentar seguir al hijo correspondiente
                match node.children[bit as usize] {
                    Trie::NonEmpty(ref child) => {
                        node = child; // Avanzar al siguiente nodo
                        if let Some(ref iface) = node.iface {
                            best_iface = Some(iface.clone()); // Actualizar la mejor coincidencia encontrada
                        }
                    }
                    Trie::Empty => {
                        break;
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


    pub fn ifaces(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        self.collect_ifaces(&mut result);
        result
    }


    fn collect_ifaces(&self, result: &mut Vec<String>) {
        match self {
            Empty => {}
            NonEmpty(ref node ) => {
                if let Some(iface) = node.iface.clone() {
                    result.push(iface.clone());
                }
                node.children[0].collect_ifaces(result);
                node.children[1].collect_ifaces(result);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_insert_exists() {
        // Tabla de encaminamiento
        let mut trie = Trie::new();


        // Entradas para la tabla de encaminamiento [ (ip, mask, iface) ]
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


    #[test]
    fn test_ifaces() {
        // Crear un Trie vacío
        let mut trie = Trie::new();


        // Insertar rutas en el Trie
        let entries = [
            (Ipv4Addr::new(192, 168, 0, 0), 24, String::from("eth0")),
            (Ipv4Addr::new(127, 0, 0, 0), 24, String::from("lo")),
            (Ipv4Addr::new(192, 168, 0, 32), 27, String::from("eth2")),
            (Ipv4Addr::new(192, 168, 0, 2), 32, String::from("eth1")),
        ];


        for (ip, mask, iface) in &entries {
            trie.insert(*ip, *mask, iface.clone()).unwrap();
        }


        // Llamar al método ifaces
        let collected_ifaces = trie.ifaces();


        // Verificar que todas las interfaces se hayan recopilado correctamente
        let expected_ifaces = vec![
            String::from("eth0"),
            String::from("lo"),
            String::from("eth2"),
            String::from("eth1"),
        ];


        assert_eq!(collected_ifaces.len(), expected_ifaces.len());
        for iface in expected_ifaces {
            assert!(collected_ifaces.contains(&iface));
        }
    }
}


fn main() { }

