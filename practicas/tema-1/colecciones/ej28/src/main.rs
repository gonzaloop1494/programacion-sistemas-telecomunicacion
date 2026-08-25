
use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, net::{IpAddr, Ipv4Addr}};


#[derive(Debug, Eq, PartialEq)]
struct Task {
    prioridad: u32,
    description: String
}


impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prioridad.cmp(&other.prioridad)
    }
}


impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct Route {
    gateway: Ipv4Addr,
    interface: String,
    metric: u32,
}

fn main() {

    //1. Almacenar la frecuencia de aparición de cada palabra en un texto

    //La colección más adecuada sería el HashMap, cada palabra se correspondería con la clave, y la frecuencia con la que aparecen es el valor.
    // En el caso de que quisiéramos obtener las palabras ordenadas, emplearía un BTreeMap.
    let mut e1: HashMap<String, u32> = HashMap::new();   //String, queda mejor sintaxis, también se podría implementar un BTreeMap, con orden
    e1.insert("hola".to_lowercase(), 5);
    e1.insert("hoy".into(), 6);
    e1.insert("mañana".into(), 9);
    for e in e1 {
        println!("{:?}", e);
    }

    //2.	Almacenar las 100 últimas medidas de un termómetro que mide la temperatura cada hora

    //Utilizaría un VecDeque, guardando cada temperatura con un push_back(),
    // hasta que la cola esté llena (las 100 últimas temperaturas medidas y almacenadas).
    // Se obtendrían las temperaturas que estén almacenadas al principio con un pop_front().
    let mut e2: VecDeque<f64> = VecDeque::with_capacity(110);
    e2.push_back(2.09);
    e2.push_back(4.98);
    println!("{}", e2.pop_front().unwrap());

    //3. Almacenar los nombres de los ficheros que mandan los usuarios
    // a una impresora para que se vayan imprimiendo en el orden en el que llegaron las peticiones

    //Emplearía un VecDeque para ir guardando los nombres de los ficheros con push_back(),
    // y después imprimir los nombres comenzando por el primero que llegó, con un pop_front().
    let mut e3: VecDeque<String> = VecDeque::new();
    e3.push_back("Rust".into());

    //4. Almacenar las tareas que tiene que realizar una persona,
    // teniendo cada tarea una prioridad que indica en qué orden hay que irlas realizando

    //Usaría la colección BinaryHeap para implementar una cola de prioridades en la que se fueran almacenando las tareas,
    // y que se fueran colocando por el orden de prioridad, según el rasgo de cada tarea.


    //para hacer un BinaryHeap, hay que crear un TipoTarea
    let mut e4: BinaryHeap<Task> = BinaryHeap::new(); //hay que indicarle a Rust la prioridad del tipo que he creado
    e4.push(Task { prioridad: 2, description: "Santiago".into() });
    e4.push(Task { prioridad: 1, description: "Victor".into() });
    e4.push(Task { prioridad: 3, description: "Sergio".into() });
    println!("{:?}", e4.pop().unwrap());

    //5.	Almacenar la tabla de encaminamiento de un router

    //HashMap que almacena: Clave , que es la Ipv4Addr; la dirección IP;
    // y Valor, Route, la estructura que contiene la información asociada a la ruta.
    let mut e5: HashMap<Ipv4Addr, Route> = HashMap::new();
    e5.insert(Ipv4Addr::new(145, 0, 0, 3), Route { gateway: Ipv4Addr::new(145, 0, 0, 3), interface: "Int1".into(), metric: 5 });


    //6.    Almacenar el conjunto de las direcciones IP vecinas de una interfaz de red de un ordenador

    //Emplearía un HashSet, ya que las direcciones IP del conjunto no se pueden repetir
    // y se van almacenando como clave porque no hay ningún valor asociado.
    let mut e6: HashSet<Ipv4Addr> = HashSet::new();
    e6.insert(Ipv4Addr::new(190, 150, 1, 20));
}