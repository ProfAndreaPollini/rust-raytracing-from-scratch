use macroquad::{
    prelude::{Mat4, Vec3, Vec4},
    rand::RandomRange,
};

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_vector();

    if in_unit_sphere.dot(normal) > 0. {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

pub fn random_unit_vector() -> Vec3 {
    let a = RandomRange::gen_range(0., 2. * std::f32::consts::PI);
    let z = RandomRange::gen_range(-1., 1.) as f32;
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn near_zero(v: Vec3) -> bool {
    let s = 1e-8;

    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct PointVector {}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct DirectionVector {}

// #[derive(Debug, Clone, Copy)]
// pub struct Vector<T> {
//     pub data: Vec3,
//     kind: PhantomData<T>,
// }

// impl<T> Vector<T> {
//     pub fn new(x: f32, y: f32, z: f32) -> Self {
//         Self {
//             data: Vec3::new(x, y, z),
//             kind: PhantomData,
//         }
//     }
// }

// impl<T: PartialEq> Eq for Vector<T> {}

// impl<T: PartialEq> PartialEq for Vector<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.data == other.data
//     }
// }

// pub type Point = Vector<PointVector>;
// pub type Direction = Vector<DirectionVector>;

// impl Add<Direction> for Point {
//     type Output = Point;

//     fn add(self, rhs: Direction) -> Self::Output {
//         Point::new(
//             self.data.x + rhs.data.x,
//             self.data.y + rhs.data.y,
//             self.data.z + rhs.data.z,
//         )
//     }
// }

// impl Add<Direction> for Direction {
//     type Output = Direction;

//     fn add(self, rhs: Direction) -> Self::Output {
//         Direction::new(
//             self.data.x + rhs.data.x,
//             self.data.y + rhs.data.y,
//             self.data.z + rhs.data.z,
//         )
//     }
// }

// impl Add<Point> for Point {
//     type Output = Point;

//     fn add(self, rhs: Point) -> Self::Output {
//         Point::new(
//             self.data.x + rhs.data.x,
//             self.data.y + rhs.data.y,
//             self.data.z + rhs.data.z,
//         )
//     }
// }

// impl Sub<Point> for Point {
//     type Output = Point;

//     fn sub(self, rhs: Point) -> Self::Output {
//         Point::new(
//             self.data.x - rhs.data.x,
//             self.data.y - rhs.data.y,
//             self.data.z - rhs.data.z,
//         )
//     }
// }

// impl Sub<Direction> for Point {
//     type Output = Point;

//     fn sub(self, rhs: Direction) -> Self::Output {
//         Point::new(
//             self.data.x - rhs.data.x,
//             self.data.y - rhs.data.y,
//             self.data.z - rhs.data.z,
//         )
//     }
// }

// impl Sub<Direction> for Direction {
//     type Output = Direction;

//     fn sub(self, rhs: Direction) -> Self::Output {
//         Direction::new(
//             self.data.x - rhs.data.x,
//             self.data.y - rhs.data.y,
//             self.data.z - rhs.data.z,
//         )
//     }
// }

// impl<T> Mul<f32> for Vector<T> {
//     type Output = Vector<T>;

//     fn mul(self, rhs: f32) -> Self::Output {
//         Vector::new(self.data.x * rhs, self.data.y * rhs, self.data.z * rhs)
//     }
// }

// impl Mul<Point> for Mat4 {
//     type Output = Point;

//     fn mul(self, other: Point) -> Self::Output {
//         let v = Vec4::new(other.data.x, other.data.y, other.data.z, 1.0);
//         let result = self * v;

//         Point::new(result.x, result.y, result.z)
//     }
// }

// //get vector kind

// fn is_point(_vector: Point) -> bool {
//     _vector.kind == PhantomData::<PointVector>
// }

// fn is_direction(_vector: Direction) -> bool {
//     _vector.kind == PhantomData::<DirectionVector>
// }

// // unit tests
// #[cfg(test)]
// mod tests {
//     use macroquad::prelude::Mat4;

//     use super::*;

//     #[test]
//     fn test_point() {
//         let p = Point::new(1.0, 2.0, 3.0);
//         assert_eq!(p.data.x, 1.0);
//         assert_eq!(p.data.y, 2.0);
//         assert_eq!(p.data.z, 3.0);
//     }

//     #[test]
//     fn test_point_add_direction() {
//         let p = Point::new(1.0, 2.0, 3.0);
//         let d = Direction::new(1.0, 2.0, 3.0);
//         let p2 = p + d;

//         assert_eq!(p2.data.x, 2.0);
//         assert!(is_point(p2));
//         // assert!(is_direction(p2));
//     }

//     #[test]
//     fn test_direction_point_sub() {
//         let p = Point::new(3., 2., 1.);
//         let d = Direction::new(5., 6., 7.);
//         let p2 = p - d;

//         assert_eq!(p2.data.x, -2.);
//         assert_eq!(p2.data.y, -4.);
//         assert_eq!(p2.data.z, -6.);

//         assert!(is_point(p2));
//         // assert!(is_direction(p2));
//     }

//     #[test]
//     fn test_direction_direction_sub() {
//         let d1 = Direction::new(3., 2., 1.);
//         let d2 = Direction::new(5., 6., 7.);
//         let d3 = d1 - d2;

//         assert_eq!(d3.data.x, -2.);
//         assert_eq!(d3.data.y, -4.);
//         assert_eq!(d3.data.z, -6.);

//         // assert!(is_point(d3));
//         assert!(is_direction(d3));
//     }

//     #[test]
//     fn test_f32_mul_direction_or_vector() {
//         let d = Direction::new(1., 2., 3.);
//         let d2 = d * 2.;

//         assert_eq!(d2.data.x, 2.);
//         assert_eq!(d2.data.y, 4.);
//         assert_eq!(d2.data.z, 6.);

//         assert!(is_direction(d2));
//         // assert!(is_point(d2));
//     }

//     #[test]
//     fn test_vector_mul_mat4_identity() {
//         let v: Point = Vector::new(1., 2., 3.);
//         let m = Mat4::IDENTITY;
//         let v2 = m * v;

//         assert_eq!(v2.data.x, 1.);
//         assert_eq!(v2.data.y, 2.);
//         assert_eq!(v2.data.z, 3.);

//         assert!(is_point(v2));
//         // assert!(is_point(v2));
//     }
// }
