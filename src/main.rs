pub mod engine;
use engine::entity_components::Pos3D;

fn main() {
    engine::core::gameloop();
    engine::camera::Camera3D::new();
}

//shape function, create a function that describes a simple 3D body, like an sphere or a torus 
fn sphere(radious: usize, center: Pos3D) {

}

//define light source position


//normal function, calculate the amount of light reflected by a point

//project function, render the 3D scene into a matrix

//mapping function, map each light intesity value to a corresponding character

//draw function, plot the 2D matrix into terminal

