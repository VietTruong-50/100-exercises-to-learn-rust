enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    // TODO: Implement the `radius` method using
    //  either an `if let` or a `let/else`.
    pub fn radius(&self) -> f64 {
        if let Self::Circle { radius } = &self {
            *radius
        } 
        if let Self::Square { border } = &self {
            *border
        } 
         if let Self::Rectangle { width, height } = &self {
            width * height
        } else {
            panic!("Invalid shape")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
        .radius();
    }
}
