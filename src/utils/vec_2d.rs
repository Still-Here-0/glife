
#[derive(PartialEq, Debug)]
pub struct Vec2D {
    pub x: f64,     // Lat
    pub y: f64      // Lon
}

impl Vec2D {
    pub fn get_distance(&self, other: &Vec2D) -> f64 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_distance() {
        let p1 = Vec2D { x: 4.0, y: 0.0 };
        let p2 = Vec2D { x: 0.0, y: 3.0 };
        let distance = p1.get_distance(&p2);

        assert!(distance == 5.0);

        let p1 = Vec2D { x: 0.0, y: 4.0 };
        let p2 = Vec2D { x: 3.0, y: 0.0 };
        let distance = p1.get_distance(&p2);

        assert!(distance == 5.0);

        let p1 = Vec2D { x: 4.0, y: 0.0 };
        let p2 = Vec2D { x: 3.0, y: 0.0 };
        let distance = p1.get_distance(&p2);

        assert!(distance == 1.0);

        let p1 = Vec2D { x: 0.0, y: 4.0 };
        let p2 = Vec2D { x: 0.0, y: 3.0 };
        let distance = p1.get_distance(&p2);

        assert!(distance == 1.0);

        let p1 = Vec2D { x: 4.0, y: 0.0 };
        let p2 = Vec2D { x: 4.0, y: 0.0 };
        let distance = p1.get_distance(&p2);

        assert!(distance == 0.0);

        let p1 = Vec2D { x: 0.0, y: 4.0 };
        let p2 = Vec2D { x: 0.0, y: 4.0 };
        let distance = p1.get_distance(&p2);

        assert!(distance == 0.0);
    }
}