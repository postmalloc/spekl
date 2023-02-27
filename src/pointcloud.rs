use kiss3d::nalgebra::Point3;

pub struct PointCloud {
    points: Vec<Point3<f32>>
}

impl PointCloud {
    pub fn new() -> Self {
        PointCloud { points: Vec::new() }
    }

    pub fn add_point(&mut self, x: f32, y: f32, z: f32){
        self.points.push(Point3::new(x, y, z));
    }

    pub fn add_point3(&mut self, p: Point3<f32>){
        self.points.push(p);
    }
    
    pub fn get_points(&self) -> &Vec<Point3<f32>> {
        &self.points
    }

    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    pub fn get_point(&self, i: usize) -> Point3<f32> {
        self.points[i]
    }

    pub fn compute_bounds(&self) -> (Point3<f32>, Point3<f32>) {
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_z = f32::NEG_INFINITY;
    
        for point in &self.points {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.y > max_y {
                max_y = point.y;
            }
            if point.z < min_z {
                min_z = point.z;
            }
            if point.z > max_z {
                max_z = point.z;
            }
        }
    
        (Point3::new(min_x, min_y, min_z), Point3::new(max_x, max_y, max_z))
    }
    
}