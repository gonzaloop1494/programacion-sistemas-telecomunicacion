#[derive(Debug, PartialEq)]
struct Cola<T, const SIZE: usize> {
    a: [T; SIZE],
    first: usize,
    last: usize,
    len: usize
}


impl<T: Copy + Default, const SIZE: usize> Cola<T, SIZE> {
    fn new() -> Cola<T, SIZE> {
        Cola { a: [T::default(); SIZE], first: 0, last: 0, len: 0 }
    }

    fn len(&self) -> usize {
        self.len
    }


    fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.len == SIZE {
            return Err(String::from("Cola llena"));
        }
        self.a[self.last] = item;
        self.last = if self.last == self.a.len() - 1 { 0 } else { self.last + 1 };
        self.len += 1;
        Ok(())
    }


    fn dequeue(&mut self) -> Result<T, String> {
        if self.len == 0 {
            return Err(String::from("Cola vacía"));
        }
        let item = self.a[self.first];
        self.first = if self.first == self.a.len() - 1 { 0 } else { self.first + 1 };
        self.len -= 1;
        Ok(item)
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