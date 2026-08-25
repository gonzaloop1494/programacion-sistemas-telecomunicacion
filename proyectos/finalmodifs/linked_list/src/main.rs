#[derive(Debug)]
enum List<T> {
    Empty,
    NonEmpty(Box<ListNode<T>>)
}


#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: List<T>
}


use List::*;


impl <T: Eq + Copy> List<T> {
    fn new() -> List<T> {
        Empty
    }


    fn push_back(&mut self, new_value: T) {
        match self {
            Empty => {
                *self = NonEmpty(Box::new(ListNode { value: new_value, next: Empty }));
            }
            NonEmpty(ref mut node) => {
                node.next.push_back(new_value);
            }
        }
    }


    fn contains_r(&self, value_to_search: T) -> bool {
        match self {
            Empty => {
                false
            }
            NonEmpty(ref node) => {
                if node.value == value_to_search {
                    true
                } else {
                    node.next.contains_r(value_to_search)
                }
            }
        }
    }


    fn contains_nr(&self, value_to_search: T) -> bool {
        let mut current = self;
        while let NonEmpty(ref node) = current {
            if node.value == value_to_search {
                return true;
            }
            current = &node.next;
        }
        false
    }


    fn peek_all_accum_r(&self, out: &mut Vec<T>) {
        match self {
            NonEmpty(ref node) => {
                out.push(node.value);
                node.next.peek_all_accum_r(out);
            }
            _ => {}
        }
    }


    fn peek_all_accum_nr(&self, out: &mut Vec<T>) {
        let mut current = self;


        while let NonEmpty(ref node) = current {
            out.push(node.value);
            current = &node.next;
        }
    }


    fn peek_all(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        self.peek_all_accum_nr(&mut result);
        result
    }


    fn is_empty(&self) -> bool {
        if let Empty = self {
            true
        } else {
            false
        }
    }


    fn length_r(&self) -> usize {
        match self {
            Empty => {
                0
            }
            NonEmpty(ref node) => {
                1 + node.next.length_r()
            }
        }
    }


    fn length_nr(&self) -> usize {
        let mut result = 0;
        let mut current = self;
        while let NonEmpty(ref node) = current {
            result += 1;
            current = &node.next;
        }
        result
    }


    fn first(&self) -> Option<T> {
        if let NonEmpty(ref node) = self {
            Some(node.value)
        } else {
            None
        }
    }


    fn last_r(&self) -> Option<T> {
        match self {
            Empty => {
                None
            }
            NonEmpty(ref node) => {
                match node.next {
                    Empty => {
                        Some(node.value)
                    }
                    NonEmpty(_) => {
                        node.next.last_r()
                    }
                }
            }
        }
    }


    fn last_nr(&self) -> Option<T> {
        let mut result: Option<T> = None;
        let mut current = self;
        while let NonEmpty(ref node) = current {
            result = Some(node.value);
            current = &node.next;
        }
        result
    }


    fn get_r_aux(&self, current_index: usize, index: usize) -> Option<T> {
        match self {
            Empty => {
                None
            }
            NonEmpty(ref node) => {
                if current_index == index {
                    return Some(node.value);
                }
                node.next.get_r_aux(current_index + 1, index)
            }
        }
    }


    fn get_r(&self, index: usize) -> Option<T> {
        self.get_r_aux(0, index)
    }


    fn get_nr(&self, index: usize) -> Option<T> {
        let mut current_index: usize = 0;
        let mut current = self;
        while let NonEmpty(ref node) = current {
            if current_index == index {
                return Some(node.value);
            }
            current_index += 1;
            current = &node.next;
        }
        None
    }
}


fn main() {
    let mut ll: List<u8> = List::new();
    println!("Is empty? {}", ll.is_empty());
    println!("First: {:?}", ll.first());
    println!("Last R: {:?}", ll.last_r());
    println!("Last NR: {:?}", ll.last_nr());
    ll.push_back(7);
    ll.push_back(10);
    ll.push_back(25);
    println!("{:?}", ll);
    println!("Is empty? {}", ll.is_empty());
    println!("First: {:?}", ll.first());
    println!("Last R: {:?}", ll.last_r());
    println!("Last NR: {:?}", ll.last_nr());
    println!("Contains R 7?: {}", ll.contains_r(7));
    println!("Contains NR 7?: {}", ll.contains_nr(7));
    println!("Contains R 10?: {}", ll.contains_r(10));
    println!("Contains NR 10?: {}", ll.contains_nr(10));
    println!("Contains R 25?: {}", ll.contains_r(25));
    println!("Contains NR 25?: {}", ll.contains_nr(25));
    println!("Contains R 4?: {}", ll.contains_r(4));
    println!("Contains NR 4?: {}", ll.contains_nr(4));
    println!("{:?}", ll.peek_all());
    println!("Length R: {}", ll.length_r());
    println!("Length NR: {}", ll.length_nr());
    println!("Get R: {:?}", ll.get_r(0));
    println!("Get R: {:?}", ll.get_r(1));
    println!("Get R: {:?}", ll.get_r(2));
    println!("Get R: {:?}", ll.get_r(3));
    println!("Get NR: {:?}", ll.get_nr(0));
    println!("Get NR: {:?}", ll.get_nr(1));
    println!("Get NR: {:?}", ll.get_nr(2));
    println!("Get NR: {:?}", ll.get_nr(3));
}

