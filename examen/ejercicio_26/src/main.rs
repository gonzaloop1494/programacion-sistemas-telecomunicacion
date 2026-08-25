struct Meters(f64); //struct tupla

#[derive(Debug, PartialEq)]
struct MetersSquared(f64); //struct tupla

#[derive(Debug, PartialEq)]
struct YardsSquared(f64);

trait HasArea { //rasgo a implementar
    fn area_m2(&self) -> MetersSquared; //al asumir la característica HasArea, hay que darle cuerpo a esta función
}

struct AreaCalculator { //struct calculador de área, que le da valor en metros cuadrados
    area_m2: MetersSquared,
}

impl AreaCalculator {
    fn new() -> Self {
        AreaCalculator {
            area_m2: MetersSquared(0.0), //Inicializa a 0 el struct tupla
        }
    }

    fn add(&mut self, shape: impl HasArea) { //Operación suma, y se recibe una figura, que tiene que implementar el trait HasArea
                                             //para este caso, Square o Rectangle
        self.area_m2 = MetersSquared(self.area_m2.0 + shape.area_m2().0); //actualiza el valor del área en m2 con un valor construido
                                                                        //por el MetersSquared, donde cogen del área que ya teníamos
                                                                        //le suman de nuestra figura, el cálculo del área
                                                                        //el .0 es para acceder a su primera porción (por ser STRUCT TUPLA)
    }

    fn total_m2(&self) -> MetersSquared { //Recibe una referencia a nuestro calculador de área, y devuelve el valor en m2
                                          //a modo de consulta
        MetersSquared(self.area_m2.0)
    }

    fn total_y2(&self) -> YardsSquared { //área total en yardas cuadradas
        YardsSquared(self.total_m2().0 * 1.19599) //el .0 para acceder al struct tupla
    }
}

//mirando en los tests se deduce que hay que crear un struct Square
struct Square {
    side: Meters, //esto tiene que implementar el TRAIT HasArea
}
impl HasArea for Square {
    fn area_m2(&self) -> MetersSquared { //misma cabecera que la función area_m2 anterior
        MetersSquared(self.side.0 * self.side.0)   //devolvería f64 en vez de un MetersSquared
                                    //.0 debido a ser un struct tupla
    }
}
//Ahora hay que crear el tipo rectángulo
struct Rectangle {
    width: Meters,
    height: Meters,
}
impl HasArea for Rectangle {
    fn area_m2(&self) -> MetersSquared {
        MetersSquared(self.width.0 * self.height.0) //struct tupla creada al principio -> MetersSquared(f64)
    }
}


#[test]
fn t_m2_1_figure() {
    let mut ac = AreaCalculator::new();
    assert_eq!(ac.total_m2(), MetersSquared(0.0)); //de primeras, SQUARE Y RECTANGLE no existían

    ac.add(Square { side: Meters(2.0) });
    assert_eq!(ac.total_m2(), MetersSquared(4.0));
}

#[test]
fn t_m2_2_figures() {
    let mut ac = AreaCalculator::new();
    assert_eq!(ac.total_m2(), MetersSquared(0.0));

    ac.add(Square { side: Meters(2.0) });
    assert_eq!(ac.total_m2(), MetersSquared(4.0));

    ac.add(Rectangle {
        width: Meters(2.0),
        height: Meters(3.0),
    });
    assert_eq!(ac.total_m2(), MetersSquared(10.0));
}

#[test]
fn test_yards_squared_2_figures() {
    let mut ac = AreaCalculator::new();
    assert_eq!(ac.total_y2(), YardsSquared(0.0));

    ac.add(Square { side: Meters(2.0) });
    assert_eq!(ac.total_y2(), YardsSquared(4.78396));

    ac.add(Rectangle {
        width: Meters(2.0),
        height: Meters(3.0),
    });
    assert_eq!(ac.total_y2(), YardsSquared(11.959900000000001));
}

fn main() {}