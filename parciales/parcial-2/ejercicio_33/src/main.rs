//Requiere que los valores de tipo T se puedan comparar
//T tiene que implementar el trait Ord
//para insertar un valor en el TreeNode: t.insert();
pub enum BinaryTree<T> {  //enumerado que representa un árbol de búsqueda binaria
    Empty, //puede estar vacío
    NonEmpty(Box<TreeNode<T>>), //o No vacío, y tener un puntero inteligente a un TreeNode (TreeNode apunta a otros
                                //dos Treenode hijos
}
pub struct TreeNode<T> { //Nodo Árbol
    value: T, //valor de tipo genérico
    left: BinaryTree<T>, //árbol binario hijo izquierdo,  que apunta a otro TreeNode con valores menores
    right: BinaryTree<T>, //árbol binario hijo derecho, que apunta a otro TreeNode con valores mayores
}


use BinaryTree::*; //evita tener que acceder todo el rato a los bloques del enum de forma explícita


impl<T: Ord + Clone> BinaryTree<T> { //el enum BinaryTree requiere que sus valores de tipo T se puedan comparar y clonar
    pub fn new() -> BinaryTree<T> { //construimos el Tree vacío
        Empty
    }
//función auxiliar
//implementación de una función que va recolectando los valores de un Árbol Binario en un vector dinámico (accum)
    pub fn peek_all_accum(&self, accum: &mut Vec<T>, ascending: bool) {
    //si ascending es True, devuelve un Vec con los valores de los nodos en orden de menor a mayor
    // Si ascending es False, devuelve un Vec con los valores de los nodos en orden de mayor a menor

        match self {
            Empty => {} //si el Tree está vacío, no recolecta nada
            NonEmpty(ref node) => { //si el Tree No está Vacío (tendrá un puntero al siguiente Tree)
                if ascending { // si los queremos de menor a mayor
                    node.left.peek_all_accum(accum, ascending); //primero acumulo los valores de las ramas izquierdas
                    //para recolectar de forma recursiva(de abajo a arriba los valores)
                    accum.push(node.value.clone()); //voy almacenando los valores en el vector dinámico accum
                    //y clonando dichos valores para no perderlos del árbol

                    //cuando ya tengo recolectados todos los valores izquierdos de abajo a arriba
                    node.right.peek_all_accum(accum, ascending); //recolecto ahora los valores de las ramas derecha, que son los mayores
                    // de forma recursiva
                } else { //si los quiero de mayor a menor (orden descendente), hago lo contrario)
                    node.right.peek_all_accum(accum, ascending); //guardo primero de abajo a arriba los valores de las ramas derechas
                    accum.push(node.value.clone());
                    node.left.peek_all_accum(accum, ascending);//luego recolecto los valores menores, de las ramas izquierdas
                }
            }
        }
    }

//función que devuelve el vector dinámico con los valores ordenados (dependiendo del valor de ascending booleano)
    pub fn peek_all(&self, ascending: bool) -> Vec<T> {
        let mut accum: Vec<T> = Vec::new();
        self.peek_all_accum(&mut accum, ascending);
        accum
    }

//implementación para insertar un valor en un Tree
//le paso como parámetros el Tree con referencia mutable y el valor que se desea insertar
    pub fn insert(&mut self, new_value: T) {
        match self { //compruebo si el Tree en el que voy a insertar está vacío o no
            Empty => { //NODO HOJA, he llegado al final, inserto aquí el valor
                *self = NonEmpty(Box::new(TreeNode { //cambio el Tree de Empty a NonEmpty,
                                                //creo puntero inteligente que apunte al sig tree
                    value: new_value, //inserto el nuevo valor al árbol
                    left: Empty, //suponemos que los Trees hijos están vacíos, son los últimos
                    right: Empty,
                }))
            }
            NonEmpty(ref mut node) => { //NO VACÍO, ref mut puntero inteligente al siguiente Tree
                                                            //desrefenciación puntero, accedo al valor del sig tree
                if new_value <= node.value { //el valor del nodo en el que estoy es mayor o igual del que quiero insertar??
                    node.left.insert(new_value); //lo inserto en el nodo hijo izquierdo
                } else { //si el valor del nodo en el que estoy es menor del que quiero insertar
                    node.right.insert(new_value); //lo inserto en el tree hijo derecho
                }
            }
        }
    }

//implementación para comprobar que existe un valor en el árbol binario
    pub fn exists(&self, value: T) -> bool {
        match self {
            BinaryTree::Empty => false, //si el nodo está vacío, no hay valor, ni ese ni ninguno
            //aquí termina la búsqueda y se devulve False

            BinaryTree::NonEmpty(ref node) => { //si no está vacío (contiene puntero a Nodos hijos)
                if value == node.value { //si el valor que buscamos es igual nodo en el que estamos
                    true
                } else if value < node.value { //si el valor que busco es menor del nodo en el que estoy
                    node.left.exists(value) //búsqueda recursiva por los nodos hijos izquierdos
                } else {
                    node.right.exists(value) //búsqueda recursiva por los nodos hijos izquierdos
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_insert_exists() {
        let mut tree = super::BinaryTree::new();


        tree.insert("Putin");
        tree.insert("Jinping");
        tree.insert("Feijoo");
        tree.insert("Sánchez");
        tree.insert("Trump");


        assert_eq!(true, tree.exists("Trump"));
        assert_eq!(false, tree.exists("Clinton"));
    }


    #[test]
    fn peek_all_ascending() {
        let mut tree = super::BinaryTree::new();


        tree.insert("Putin");
        tree.insert("Jinping");
        tree.insert("Feijoo");
        tree.insert("Sánchez");
        tree.insert("Trump");


        assert_eq!(vec!["Feijoo", "Jinping", "Putin", "Sánchez", "Trump"], tree.peek_all(true));
    }


    #[test]
    fn peek_all_descending() {
        let mut tree = super::BinaryTree::new();


        tree.insert("Putin");
        tree.insert("Jinping");
        tree.insert("Feijoo");
        tree.insert("Sánchez");
        tree.insert("Trump");


        assert_eq!(vec!["Trump", "Sánchez", "Putin", "Jinping", "Feijoo"], tree.peek_all(false));
    }
}


fn main() { }
