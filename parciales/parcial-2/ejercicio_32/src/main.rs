// Una lista (List) o está vacía; o es un nodo (Node), que contiene un valor(u8), y un List (next)


enum List { //es una enumeración que modela una lista enlazada (linked list)
    Empty, //representa el caso base o el final de una lista, indicando que está vacía
    NonEmpty(Box<Node>), //lista no vacía, que contiene un puntero al siguiente nodo de la lista
                         // lo que hace que la lista esté vacía
}


use List::*; //si no pongo esto, tendría que poner list::empty
// permite importar todas las variantes (Empty y NonEmpty) al espacio de nombres actual


//es el nodo al que apunta el puntero NonEmpty
//contiene un valor (u8) y una Lista (next)
struct Node {
    value: u8,
    next: List,
}
//si el nodo siguiente tiene una lista Nonempty, apunta al NonEmpty <List> y al valor (de la caja siguiente)

impl List {
    fn new() -> List {
        Empty
    }


    fn exists(&self, value: u8) -> bool { //se le pasa una lista y un valor
        match self { //busca si hay algún valor dentro de la lista
            Empty => false,
            //el tipo del node es &Box<node>
            NonEmpty(ref node) => {
                if node.value == value {
                    true
                } else {
                    node.next.exists(value) //el objeto es la lista, y el puntero al valor del nodo siguiente
                }
            }
        }
    }


    // Ojo: esta implementación es muy ineficiente:
    // añade un elemento al final recorriendo todos los elementos hasta llegar
    // al último:
    fn push(&mut self, new_value: u8) {
        match self { // desreferencia el puntero
            Empty => {
                *self = NonEmpty(Box::new(Node {
                    value: new_value,
                    next: Empty,
                }));
            }
            NonEmpty(ref mut node) => {
                node.next.push(new_value);
            }
        }
    }


    fn peek_all_accum(&self, accum: &mut Vec<u8>) { //función auxiliar para recolectar los elementos de la lista enlazada
        match self {
            Empty => {}, //si está vacía, no se hace nada
            NonEmpty(ref node) => { //si no está vacía,
                accum.push(node.value); //se guarda el valor del nodo en la variable mutable accum, que ES UN VECTOR DINÁMICO
                node.next.peek_all_accum(accum); //se pasa al siguiente nodo al que apunta la lista, de manera recursiva,
                // con accum como variable que se pasa a la función
            }
        }
    }


    fn peek_all(&self) -> Vec<u8> { //función que devuelve en forma de vector dinámico los valores de una lista enlazada
        let mut accum: Vec<u8> = Vec::new();
        self.peek_all_accum(&mut accum);
        return accum;
    }
}


#[cfg(test)]
mod tests {


    #[test]
    fn test_push_exists() {
        let mut l = super::List::new();  //super se utiliza para referirse al nivel superior (o módulo padre) del módulo actual
        l.push(3);
        l.push(7);
        l.push(1);

        assert_eq!(false, l.exists(25));
        assert_eq!(true, l.exists(1));

        assert_eq!(true, l.exists(3));
        assert_eq!(true, l.exists(7));

    }


    #[test]
    fn test_peek_all() {
        let mut l = super::List::new();
        l.push(3);
        l.push(7);
        l.push(1);

        assert_eq!(vec![3,7,1], l.peek_all());
    }
}


fn main() { }

