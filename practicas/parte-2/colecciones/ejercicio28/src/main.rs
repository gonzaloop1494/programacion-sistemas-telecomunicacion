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
    // 1
    let mut e1: HashMap<String, u32> = HashMap::new(); // let mut e1: BTreeMap<String, u32> = BTreeMap::new(); con orden
    e1.insert("DOG".to_lowercase(), 5);
    e1.insert("cat".into(), 7);
    e1.insert("bird".into(), 10);
    for e in e1 {
        println!("{:?}", e);
    }


    // 2
    let mut e2: VecDeque<f64> = VecDeque::with_capacity(100);
    e2.push_back(3.75);
    e2.push_back(10.5);
    e2.push_back(18.0);
    println!("{}", e2.pop_front().unwrap());


    // 3
    let mut e3: VecDeque<String> = VecDeque::new();
    e3.push_back("Imperio Romano".into());
    e3.push_back("PST".into());
    // ...


    // 4
    let mut e4: BinaryHeap<Task> = BinaryHeap::new();
    e4.push( Task { prioridad: 2, description: "Santiago".into() } );
    e4.push( Task { prioridad: 1, description: "Victor".into() } );
    e4.push( Task { prioridad: 3, description: "Sergio".into() } );
    println!("{:?}", e4.pop().unwrap());


    // 5
    let mut e5: HashMap<Ipv4Addr, Route> = HashMap::new();
    e5.insert(Ipv4Addr::new(127, 0, 0, 1), Route { gateway: Ipv4Addr::new(127, 0, 0, 1), interface: "I1".into(), metric: 5 });
    // ...


    // 6
    let mut e6: HashSet<Ipv4Addr> = HashSet::new();
    e6.insert(Ipv4Addr::new(192, 168, 1, 25));
    // ...
}

