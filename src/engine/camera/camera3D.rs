use std;
use super::super::entity_components::{Pos3D,Vec3D};
const H_FOV: u8 = 103;
const V_FOV: u8 = 71;

struct Camera3D {
    pub pos: Pos3D,
    pub dir: Vec3D,
    dist: f32,

    render_size: (usize,usize),
    ray_origins: Vec<Vec3D>,
}

impl Camera3D {
    pub fn new(pos: Pos3D, dir: Vec3D, render_size: (usize,usize)) -> Camera3D {
        let dist= (H_FOV as f32).to_radians().tan() * render_size.0 as f32/2.;
        let ray_origins = Vec::<Vec3D>::with_capacity(render_size.0 as usize * render_size.1 as usize); //alocate

        Camera3D { pos, dir: dir.normalize(), dist, render_size, ray_origins}
    }

    fn render(&mut self, render_size: (usize,usize)) {
        const V: Vec3D = Vec3D::y(); 
        // being V a the canonical vector in y direction
        // being dir the vector that points towards the center of the rendering plane
        // if v and dir form basis, then ortho_v is the orthonormalized version of that basis  
        let H =  (V - V.dot(&self.dir) / self.dir.norm_squared() * self.dir ).normalize();
        let W = H.cross(&self.dir).normalize(); //R3 cross product is enough to get the third basis
        // {W, H, dir} is the most convient basis to generate the rays in each render cycle because W and H are the vector for the X and Y
        // coordinates realtive to the camera direction, in other words the screen coordinates
        let basis = (W,H,self.dir);
        
        let half_width: isize = render_size.0 as isize / 2;
        let half_height: isize = render_size.1 as isize/ 2;

        for h_index in 0..render_size.0 - 1 {
            for w_index in 0..render_size.1 - 1{
                let ray_index = render_size.0 * h_index + w_index;
                self.ray_origins[ray_index][0] = 
                self.ray_origins[ray_index][1] =
                self.ray_origins[ray_index][2] =
            }
        }
    }
}
