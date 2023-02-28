pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3 { x, y, z }
    }
}

impl<T: Copy> Copy for Point3<T> {}

impl<T: Clone> Clone for Point3<T> {
    fn clone(&self) -> Self {
        Point3 {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}