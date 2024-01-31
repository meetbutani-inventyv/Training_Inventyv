/// Creating Shapes structure to store the dimensions
struct Shapes {
    length: f32,
    breadth: f32
}

impl Shapes {
    fn calc_area_of_circle(&self) -> f32 {
        3.14 * (self.length/2.0) * (self.length/2.0)
    }

    fn calc_area_of_square(&self) -> f32 {
        self.length * self.length
    }

    fn calc_area_of_rectangle(&self) -> f32{
        self.length * self.breadth
    }
}

fn main() {
    let shape = Shapes {
        length: 10.0,
        breadth: 15.0
    };

    println!("Area of circle is: {}", shape.calc_area_of_circle());
    println!("Area of square is: {}", shape.calc_area_of_square());
    println!("Area of reactangle is: {}", shape.calc_area_of_rectangle());
}