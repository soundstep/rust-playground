// cargo script src/generic_method_definition.rs

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct AnotherPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> AnotherPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: AnotherPoint<X2, Y2>) -> AnotherPoint<X1, Y2> {
        AnotherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }
    {
        let p1 = AnotherPoint { x: 5, y: 10.4 };
        let p2 = AnotherPoint { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
