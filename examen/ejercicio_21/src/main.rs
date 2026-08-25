#[derive(Copy, Clone)]  //derivación de dos rasgos, en este caso Copy y Clone (CLONADO AUTOMÁTICO)
struct Square {
    width: i32,
}


impl Square {    //implementación del struct
    fn area(&self) -> i32 {     //self se pasaba por copia antes, no por referencia. Al terminar de ejecutarse la función desaparecería
        self.width * self.width      //pasando el cuadrado por referencia en este caso no soluciona el problema, la variable s quedaría inutilizada
                                     //al hacer la asignación
    }
}

#[test]
fn t() {
    let s = Square { width: 2 }; //Sé que es un struct porque tengo un tipo con una característica entre llaves
                                 //a la cual se le da un valor (ESTRUCTURA DE STRUCT)
    let s2 = s;          // PROBLEMA DE TRANSFERENCIA O PROPIETARIO: la propiedad de Struct, al hacer la asignación,
                                // pasa de s a s2, s la pierde. Y en posteriores llamadas de estas variables, ya no serán las propietarias
                                 //Para que esto no pase, se añade
    //como he pasado por referencia el cuadrado a la función área, no entra en juego aquí el CLONADO AUTOMÁTICO
    assert_eq!(4, s.area());
    assert_eq!(4, s2.area());

    assert_eq!(4, s.area());
    assert_eq!(4, s2.area());
}

fn main() {}