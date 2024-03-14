struct Point2D {
    x: f64,
    y: f64,
}

struct Triangle2D {
    point_a: Point2D,
    point_b: Point2D,
    point_c: Point2D,
}

impl Triangle2D {
    fn is_point_inside(&self, point: &Point2D) -> bool {
        let area = 0.5
            * (-self.point_b.y * self.point_c.x
                + self.point_a.y * (-self.point_b.x + self.point_c.x)
                + self.point_a.x * (self.point_b.y - self.point_c.y)
                + self.point_b.x * self.point_c.y);
        let s = 1.0 / (2.0 * area)
            * (self.point_a.y * self.point_c.x - self.point_a.x * self.point_c.y
                + (self.point_c.y - self.point_a.y) * point.x
                + (self.point_a.x - self.point_c.x) * point.y);
        let t = 1.0 / (2.0 * area)
            * (self.point_a.x * self.point_b.y - self.point_a.y * self.point_b.x
                + (self.point_a.y - self.point_b.y) * point.x
                + (self.point_b.x - self.point_a.x) * point.y);

        s >= -0.0001 && t >= -0.0001 && (s + t) <= 1.0001
    }
}

fn main() {
    println!("Hello, world!");
}
