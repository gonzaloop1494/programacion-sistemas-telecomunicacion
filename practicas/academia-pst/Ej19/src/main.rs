struct PoligonoRegular {
    numero_lados: i32,
    longitud_lado: i32,
}
impl PoligonoRegular {
    fn new(numero_lados: i32, longitud_lado: i32) -> PoligonoRegular {
        PoligonoRegular {
            numero_lados,
            longitud_lado
        }
    }
    fn perimetro(&self) -> u32 {
        (self.numero_lados as u32) * (self.longitud_lado as u32)
    }
}
#[test]
fn perimetro_triangulo() {
    let mut t = PoligonoRegular {
        numero_lados: 3,
        longitud_lado: 10,
    };
    assert_eq!(t.perimetro(), 30);

    t.longitud_lado = 20;
    assert_eq!(t.perimetro(), 60);
}

#[test]
fn perimetro_cuadrado() {
    let c = PoligonoRegular::new(10, 4);

    assert_eq!(c.perimetro(), 40);
}
