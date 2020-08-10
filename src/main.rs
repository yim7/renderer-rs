mod canvas;
mod color;
mod interpolate;
mod matrix;
mod mesh;
mod texture;
mod vector;
mod vertex;
mod window;
use mesh::Mesh;
use vector::Vector;
use window::Window;

pub fn main() {
    let mut mesh = Mesh::load_obj("assets/ahri.obj", Some("assets/ahri.image")).unwrap();
    mesh.set_position(Vector::new(0.0, 0.0, -10.0));
    let mut window = Window::new("demo", 800, 600).unwrap();
    window.set_mesh(mesh);
    window.run();
}
