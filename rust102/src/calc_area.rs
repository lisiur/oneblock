pub trait AreaCalcable {
    fn calc_area(&self) -> f32;
}

pub struct Circle {
    r: f32,
}
impl Circle {
    pub fn new(r: f32) -> Self {
        Self {
            r
        }
    }
}
impl AreaCalcable for Circle {
    fn calc_area(&self) -> f32 {
        std::f32::consts::PI * self.r * self.r
    }
}

pub struct Square {
    a: f32,
}
impl Square {
    pub fn new(a: f32) -> Self {
        Self {
            a
        }
    }
}
impl AreaCalcable for Square {
    fn calc_area(&self) -> f32 {
        self.a * self.a
    }
}

pub struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}
impl Triangle {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self {
            a,
            b,
            c,
        }
    }
}
impl AreaCalcable for Triangle {
    fn calc_area(&self) -> f32 {
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }
}

fn calc_area<T: AreaCalcable>(subject: T) -> f32 {
    subject.calc_area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_area_test_circle() {
        let r = 2.0;
        let circle = Circle::new(r);
        let area = circle.calc_area();
        let answer = std::f32::consts::PI * 4.0;
        assert_eq!(area, answer);
    }

    #[test]
    fn calc_area_test_square() {
        let a = 2.0;
        let square = Square::new(a);
        let area = square.calc_area();
        let answer =  4.0;
        assert_eq!(area, answer);
    }

    #[test]
    fn calc_area_test_triangle() {
        let a = 3.0;
        let b = 4.0;
        let c = 5.0;
        let triangle = Triangle::new(a, b, c);
        let area = triangle.calc_area();
        let answer =  6.0;
        assert_eq!(area, answer);
    }
}
