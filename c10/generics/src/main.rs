fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct MixyPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MixyPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixyPoint<X2, Y2>) -> MixyPoint<X1, Y2> {
        MixyPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }
    {
        let p = Point { x: 5.0, y: 10.0 };
        println!("dist = {}", p.distance_from_origin());
    }
    {
        let p1 = MixyPoint { x: 5, y: 10.4 };
        let p2 = MixyPoint { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
