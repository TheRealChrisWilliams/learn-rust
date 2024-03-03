use std::fmt;

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Display for Point3D {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {write!(f, ", ")?;};
            write!(f, "{}:{}", v, count)?;
        }

        write!(f, "]")
    }
}

fn main() {
    println!("Hello, world!");
    let point = Point3D {x:5.24, y:7.19, z:9.22};
    println!("Display: {}", point);

    let array = List(vec![1, 2, 3, 4, 5]);
    println!("Array: {}", array);
}
