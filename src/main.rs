use spekl::pointcloud::PointCloud;
use spekl::visualization::render::PointcloudRenderer;
use spekl::io::ply_reader::parse_ply_file;

fn main() {
    let mut pc: PointCloud = PointCloud::new();
    let path = "data/bunny.ply";
    let pts = match parse_ply_file(path) {
        Ok(pts) => pts,
        Err(err) => {
            eprintln!("Error parsing PLY file '{}': {}", path, err);
            std::process::exit(1);
        }
    };

    for pt in pts.iter() {
        pc.add_point3(*pt);
    }

    println!("Number of points in pc: {}", pc.num_points());
    let mut renderer = PointcloudRenderer::new();
    renderer.render(&pc);
}
