// Una lista (List) o está vacía; o es un nodo (Node), que contiene un valor(u8), y un List (next)


enum List { //es una enumeración que modela una lista enlazada (linked list)
    Empty, //representa el caso base o el final de una lista, indicando que está vacía
    NonEmpty(Box<Node>), //lista no vacía, que contiene un puntero al siguiente nodo de la lista

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

//hace lo mismo que el método peek_all_accum pero de manera no recursiva
    fn peek_all_accum_nr(&self, accum: &mut Vec<u8>) {
        let mut current = self; //current es el nodo actual
        while let NonEmpty(node) = current { //mientras que el noodo actual apunte a una lista NonEmpty
            accum.push(node.value); //guardar en el vector dinámico accum el valor del nodo
            current = &node.next; //apuntar al siguiente nodo
        }
    }


    fn peek_all_accum(&self, accum: &mut Vec<u8>) { //función auxiliar para recolectar los elementos de la lista enlazada
        match self {
            NonEmpty(ref node) => { //si no está vacía,
                accum.push(node.value); //se guarda el valor del nodo en la variable mutable accum, que ES UN VECTOR DINÁMICO
                node.next.peek_all_accum(accum); //se pasa al siguiente nodo al que apunta la lista, de manera recursiva,
                // con accum como variable que se pasa a la función
            }
            _ => {} //si está vacía, no se hace nada (EMPTY)
        }
    }


    fn peek_all(&self) -> Vec<u8> { //función que devuelve en forma de vector dinámico los valores de una lista enlazada
        let mut accum: Vec<u8> = Vec::new();
        self.peek_all_accum(&mut accum);
        accum
    }

    fn exists(&self, value: u8) -> bool {  //se le pasa una lista y un valor
        match self { //busca si hay algún valor dentro de la lista
            Empty => false,
            //el tipo del node es &Box<node>
            NonEmpty(ref node) => {
                if node.value == value {
                    true
                } else {
                    node.next.exists(value) //el objeto es la lista, y el puntero al valor del nodo siguiente
                } //RECURSIVIDAD: Vuelve a llamar a la función exist para el siguiente nodo con el value como parámetro
            }
        }
    }


    fn exists_nr(&self, value: u8) -> bool {
        let mut current = self; // Empezamos en la raíz de la lista
        while let NonEmpty(node) = current {
            if node.value == value {
                return true; // Si encontramos el valor, devolvemos `true`
            }
            current = &node.next; // Nos movemos al siguiente nodo
        }
        false
    }


    fn push(&mut self, new_value: u8) {
        match self { // desreferencia el puntero
            Empty => {
                *self = NonEmpty(Box::new(Node { // si la lista es Empty, se cambia a NonEmpty y
                    //se crea un puntero al siguiente nodo
                    value: new_value, //se guarda en el nodo que era Empty antes, el valor nuevo que queria guardar
                    next: Empty, //la siguiente lista creada es Empty, la última
                }));
            }
            NonEmpty(ref mut node) => { //no vacía, con nodo ref mut al siguiente nodo
                if let Empty = node.next { //si el siguiente nodo es el último
                    node.next = NonEmpty(Box::new(Node { //crear un NonEmpty en el siguiente nodo con un puntero a otro nodo nuevo Empty
                        value: new_value, //insertar en el que era Empty el nuevo valor
                        next: Empty, // el siguiente nodo que acaba de ser creado será Empty, el último
                    }));
                } else { // si el siguiente nodo era NonEmpty también
                    node.next.push(new_value); //aplicar el método push() al siguiente nodo
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_push_exists() {
        let mut l = super::List::new();
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


        assert_eq!(vec![3, 7, 1], l.peek_all());
    }




    #[test]
    fn test_peek_all_accum_nr() {
        let mut l = super::List::new();
        l.push(3);
        l.push(7);
        l.push(1);


        let mut all = Vec::new();
        l.peek_all_accum_nr(&mut all);
        assert_eq!(vec![3, 7, 1], all);
    }


    #[test]
    fn test_push_exists_nr() {
        let mut l = super::List::new();
        l.push(3);
        l.push(7);
        l.push(1);


        assert_eq!(false, l.exists_nr(25));
        assert_eq!(true, l.exists_nr(1));


        assert_eq!(true, l.exists_nr(3));
        assert_eq!(true, l.exists_nr(7));


        assert_eq!(false, l.exists_nr(42));
    }

}


fn main() { }


