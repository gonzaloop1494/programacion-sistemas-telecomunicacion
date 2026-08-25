struct Meters(f64);

#[derive(Debug, PartialEq)]
struct MetersSquared(f64);

#[derive(Debug, PartialEq)]
struct YardsSquared(f64);

trait HasArea {
    fn area_m2(&self) -> MetersSquared;
}

struct AreaCalculator {
    area_m2: MetersSquared,
}

impl AreaCalculator {
    fn new() -> Self {
        AreaCalculator {
            area_m2: MetersSquared(0.0),
        }
    }

    fn add(&mut self, shape: impl HasArea) {
        self.area_m2 = MetersSquared(self.area_m2.0 + shape.area_m2().0);
    }

    fn total_m2(&self) -> MetersSquared {
        MetersSquared(self.area_m2.0)
    }

    fn total_y2(&self) -> YardsSquared {
        YardsSquared(self.total_m2().0 * 1.19599)
    }
}


struct Square {
    side: Meters
}


impl HasArea for Square {
    fn area_m2(&self) -> MetersSquared {
        MetersSquared(self.side.0 * self.side.0)
    }
}


struct Rectangle {
    width: Meters,
    height: Meters
}


impl HasArea for Rectangle {
    fn area_m2(&self) -> MetersSquared {
        MetersSquared(self.width.0 * self.height.0)
    }
}


#[test]
fn t_m2_1_figure() {
    let mut ac = AreaCalculator::new();
    assert_eq!(ac.total_m2(), MetersSquared(0.0));

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




fn main() {
    println!("Hello, world!");
}

