



#[test]
fn test_enqueue_front_wrap() {
    let mut q: Cola<&str, 4> = Cola::new();
    let _ = q.enqueue("A");
    let _ = q.enqueue("B");

    let _ = q.enqueue_front("C");
    assert_eq!(
        q,
        Cola {
            a: ["A", "B", "", "C"],
            first: 3,
            last: 2,
            len: 3
        }
    );
}


#[test]
fn test_dequeue_back_wrap() {
    let mut q: Cola<&str, 4> = Cola::new();
    let _ = q.enqueue("A");
    let _ = q.enqueue("B");
    let _ = q.enqueue("C");
    let _ = q.enqueue("D");

    let e = q.dequeue().unwrap();
    assert_eq!(e, "A");

    let e = q.dequeue().unwrap();
    assert_eq!(e, "B");

    let e = q.dequeue_back();
    assert_eq!(e, Ok("D"));
    assert_eq!(
        q,
        Cola {
            a: ["A", "B", "C", "D"],
            first: 2,
            last: 3,
            len: 1
        }
    );
}

#[test]
fn test_enqueue_dequeue_wrap_many() {
    const SIZE: usize = 10;

    // fill the queue
    let mut q: Cola<u64, SIZE> = Cola::new();
    while q.len() < SIZE {
        let _ = q.enqueue(1);
    }
    assert_eq!(q.len(), SIZE);

    // dequeue SIZE/2 elements
    for _i in 0..(SIZE / 2) {
        let _ = q.dequeue_back();
    }

    // fill the queue again with enqueue_wrap()
    while q.len() < SIZE {
        let _ = q.enqueue_front(2);
    }

    // no more enqueues allowed
    assert_eq!(q.enqueue(1), Err(String::from("Cola llena")));

    assert_eq!(
        q,
        Cola {
            a: [1,1,1,1,1,2,2,2,2,2],
            first: 5,
            last: 5,
            len: 10
        }
    );
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


#[test]
fn test_enqueue_front_string() {
    let mut q: Cola<&str, 4> = Cola::new();

    assert_eq!(q.len(), 0);
    let _ = q.enqueue_front("hola");
    assert_eq!(q.len(), 1);

   assert_eq!(
        q,
        Cola {
            a: ["", "", "", "hola"],
            first: 3,
            last: 0,
            len: 1
        }
    );
}

fn main() {}