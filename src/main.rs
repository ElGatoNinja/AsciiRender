mod engine;

fn main() {
    println!("Hello, world!");
}

//shape function, create a function that describes a simple 3D body, like an sphere or a torus 
fn sphere(radious: usize, center: Pos3D) {

}
//define 3D body position
//define light source position
struct Pos3D {
    x:f32,
    y:f32,
    z:f32,
}

//normal function, calculate the amount of light reflected by a point

//project function, render the 3D scene into a matrix

//mapping function, map each light intesity value to a corresponding character

//draw function, plot the 2D matrix into terminal

