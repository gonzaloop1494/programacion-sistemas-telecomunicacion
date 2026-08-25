#[derive(Debug, PartialEq)] //TRAITS Debug (para imprimir en modo depuración la información de forma legible) y
                              //TRAIT PartialEq (para que se puedan comparar entre sí dos colas)
struct Cola<T, const SIZE : usize> {
    a: [T; SIZE],
    len: usize, //contamos desde 0 en adelante (NÚMERO DE ELEMENTOS)
    first: usize, //tienen índices usize para indexar elementos de un array
    last: usize, //
}
//Al implementar, el tipo de elemento T debe tener la capacidad de copiar elementos por defecto y la capacidad de copiarse
impl<T: Default + Copy, const SIZE: usize> Cola<T, SIZE> {
    fn new() -> Cola<T, SIZE> { //CONSTRUCTOR new para un struct genérico
        Cola {a: [T::default(); SIZE], len: 0, first: 0, last: 0} //Inicializar todo a 0, usando
                                                                  //función default para inicializar a 0 el tipo T
    }
    fn len(&self) -> usize {
        self.len
    }
    fn enqueue(&mut self, data: T) -> Result<(), String> { //Encolar
        if self.len == self.a.len() {
            return Err(String::from("Cola llena"));
        }
        self.a[self.last] = data; // Nos almacenamos el valor a encolar a la posición donde apunte last
        self.last = if self.last == SIZE -1 {
            0 //si el último elemento se insertó en la última posición del array, BUFFER CIRCULAR, VUELVE A APUNTAR A 0
        } else {
            self.last + 1 //si no ha llegado al final de la cola, apunta al siguiente
        };
        self.len += 1; //se incrementa en una unidad la longitud del array TRAS HABERSE ENCOLADO OTRO ELEMENTO
        Ok(()) //se devuelve ok con el unit value
    }
    fn dequeue(&mut self) -> Result<T, String> { //desencolar
        if self.len == 0 { //Si en la cola no hay nada, longitud 0, la cola está vacía
            return Err(String::from("Cola vacía")); //se devuelve el result tipo Err con el String de aviso
        }
        let data = self.a[self.first]; //se declara la variable que guarda el elemento al que apunta FIRST en mi array
                                         //Al desencolar, se extrae el primer valor que se guardó en el array
        self.first = if self.first == SIZE - 1 {
            0  //BUFFET CIRCULAR-> al desencolar, si el valor desencolado era la última posición del array,
              //se vuelve a apuntar a la posición 0 del array
        } else {self.first + 1};
        self.len -= 1;  //Al desencolar, la longitud del array decrece en una unidad

        Ok(data)
    }

}


#[test]
fn test_enqueue_string() {
    let mut q: Cola<&str, 4> = Cola::new();

    assert_eq!(q.len(), 0);
    let _ = q.enqueue("hola");
    assert_eq!(q.len(), 1);
    assert_eq!(
        q,
        Cola {
            a: ["hola", "", "", ""],
            first: 0,
            last: 1,
            len: 1
        }
    );
}

#[test]
fn test_enqueue_wrap() {
    let mut q: Cola<&str, 4> = Cola::new();
    let _ = q.enqueue("A");
    let _ = q.enqueue("B");
    let _ = q.enqueue("C");
    let _ = q.enqueue("D");

    let e = q.dequeue().unwrap();
    assert_eq!(e, "A");

    let _ = q.enqueue("E");
    assert_eq!(
        q,
        Cola {
            a: ["E", "B", "C", "D"],
            first: 1,
            last: 1,
            len: 4
        }
    );
}

#[test]
fn test_dequeue_wrap() {
    let mut q: Cola<&str, 4> = Cola::new();
    let _ = q.enqueue("A");
    let _ = q.enqueue("B");
    let _ = q.enqueue("C");
    let _ = q.enqueue("D");

    let e = q.dequeue().unwrap();
    assert_eq!(e, "A");

    let e = q.dequeue().unwrap();
    assert_eq!(e, "B");

    let _ = q.enqueue("E");
    assert_eq!(
        q,
        Cola {
            a: ["E", "B", "C", "D"],
            first: 2,
            last: 1,
            len: 3
        }
    );

    let e = q.dequeue().unwrap();
    assert_eq!(e, "C");

    let e = q.dequeue().unwrap();
    assert_eq!(e, "D");
    assert_eq!(
        q,
        Cola {
            a: ["E", "B", "C", "D"],
            first: 0,
            last: 1,
            len: 1
        }
    );

    let e = q.dequeue().unwrap();
    assert_eq!(e, "E");

    let e = q.dequeue();
    assert_eq!(e, Err(String::from("Cola vacía")));
}

#[test]
fn test_dequeue_when_empty() {
    let mut q: Cola<u64, 4> = Cola::new();

    let e = q.dequeue();
    assert_eq!(e, Err(String::from("Cola vacía")));
}

#[test]
fn test_enqueue_dequeue_many() {
    const SIZE: usize = 100;

    // fill the queue
    let mut q: Cola<u64, SIZE> = Cola::new();
    while q.len() < SIZE {
        let _ = q.enqueue(1);
    }
    assert_eq!(q.len(), SIZE);

    // dequeue SIZE/2 elements
    for _i in 0..(SIZE / 2 + 1) {
        let _ = q.dequeue();
    }

    // fill the queue again
    while q.len() < SIZE {
        let _ = q.enqueue(1);
    }

    // no more enqueues allowed
    assert_eq!(q.enqueue(1), Err(String::from("Cola llena")));
}

fn main() {}