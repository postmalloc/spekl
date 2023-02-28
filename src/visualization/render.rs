use crate::objects::pointcloud::PointCloud;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::Point3;
use kiss3d::window::Window;

pub struct PointcloudRenderer {
    window: Window,
}

impl PointcloudRenderer {
    pub fn new() -> Self {
        let mut window = Window::new("Point Cloud Renderer");
        window.set_light(Light::StickToCamera);
        PointcloudRenderer { window }
    }

    pub fn render(&mut self, cloud: &PointCloud) {
        let num_points = cloud.num_points();
        let mut arcball = ArcBall::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        );

        let bounds = cloud.compute_bounds();
        let min_x = bounds.0.x;
        let min_y = bounds.0.y;
        let min_z = bounds.0.z;
        let max_x = bounds.1.x;
        let max_y = bounds.1.y;
        let max_z = bounds.1.z;

        let center_x = (max_x + min_x) / 2.0;
        let center_y = (max_y + min_y) / 2.0;
        let center_z = (max_z + min_z) / 2.0;
        let center = Point3::new(center_x, center_y, center_z);
        let max_dim = max_x - min_x;
        let max_dim = if max_dim < max_y - min_y {
            max_y - min_y
        } else {
            max_dim
        };
        let max_dim = if max_dim < max_z - min_z {
            max_z - min_z
        } else {
            max_dim
        };
        let radius = max_dim / 2.0;
        let eye = Point3::new(center_x, center_y, center_z + radius * 3.0);
        arcball.look_at(eye, center);

        while self.window.render_with_camera(&mut arcball) {
            for i in 0..num_points {
                let p = cloud.get_point(i);
                let color = Point3::new(1.0, 1.0, 1.0);
                self.window.draw_point(&Point3::new(p.x, p.y, p.z), &color);
            }
        }
    }
}
