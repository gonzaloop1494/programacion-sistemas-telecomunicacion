enum Figuras {
        Cuadrado {lado : f64},
        Rectangulo {base: f64, altura: f64},
}

impl Figuras {
    fn area(&self) -> f64 {
        match self {
            Figuras::Cuadrado { lado } => {
                lado * lado
            },
            Figuras::Rectangulo { base, altura } => {
                base * altura
            }
        }
    }
    fn perimetro(&self) -> f64 {
        match self {
            Figuras::Cuadrado{lado} => {
                lado * 4.0
            }
            Figuras::Rectangulo{base, altura} => {
                base * 2.0 + altura * 2.0
            }
        }

    }
    fn zoom(&mut self, zoom: f64) {
        match self {
            Figuras::Cuadrado{lado} => {
                *lado = *lado * zoom // lado *= zoom     //*lado nos permite atravesar el puntero, dirigirme a la caja a la que está
                                                        //apuntando este puntero mutable
            }
            Figuras::Rectangulo{base, altura} => {
                *base *= zoom; //*base atraviesa el puntero mutable para modificar el valor al que apunta
                *altura *= zoom;
            }
        }
    }
}

#[test]
fn test_perimetros() {
    let figuras = [   //Se trata de un enumerado
        Figuras::Cuadrado { lado: 5.0 },  //struct cuadrado, que tiene lado f64
        Figuras::Rectangulo {  //struct rectángulo, que tiene base y altura, f64 ambos
            base: 10.0,
            altura: 2.0,
        },
        Figuras::Cuadrado { lado: 3.0 },
    ];

    let mut perimetros = [0.0; 3];
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }

    assert_eq!(perimetros, [20.0, 24.0, 12.0]);
}

#[test]
fn test_areas() {
    let figuras = [
        Figuras::Cuadrado { lado: 5.0 },
        Figuras::Rectangulo {
            base: 10.0,
            altura: 2.0,
        },
        Figuras::Cuadrado { lado: 3.0 },
    ];

    let mut areas = [0.0; 3];
    for i in 0..figuras.len() {
        areas[i] = figuras[i].area();
    }

    assert_eq!(areas, [25.0, 20.0, 9.0]);
}


#[test]
fn test_zoom_perimetros() {
    let mut figuras = [
        Figuras::Cuadrado { lado: 5.0 },
        Figuras::Rectangulo {
            base: 10.0,
            altura: 2.0,
        },
        Figuras::Cuadrado { lado: 3.0 },
    ];

    let mut perimetros = [0.0; 3];
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }
    assert_eq!(perimetros, [20.0, 24.0, 12.0]);

    for i in 0..figuras.len() {
        figuras[i].zoom(2.0); // El método zoom multiplica por el parámetro cada lado de la figura
    }
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }
    assert_eq!(perimetros, [40.0, 48.0, 24.0]);


    for i in 0..figuras.len() {
        figuras[i].zoom(0.5); // El método zoom multiplica por el parámetro cada lado de la figura
    }
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }

    assert_eq!(perimetros, [20.0, 24.0, 12.0]);

}

fn main() {}