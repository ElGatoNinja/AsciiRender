use std;
use super::super::entity_components::{Pos3D,Vec3D};
const H_FOV: u8 = 103;
const V_FOV: u8 = 71;

struct Camera3D {
    pub pos: Pos3D,
    pub dir: Vec3D,
    dist: f32,

    render_size: (u8,u8),
    raycast_origins: Box<Vec3D>,
}

impl Camera3D {
    pub fn new(&mut self ,pos: Pos3D, dir: Vec3D, render_size: (u8,u8)) {
        self.pos = pos;
        self.dir = dir.normalize();
        self.render_size = render_size;
        self.dist= (H_FOV as f32).to_radians().tan() * self.render_size.0 as f32/2f32;

        self.local_raycast_coordinates = Box::new(vec![Vec3D::new(),render_size.0 * render_size.1].into_boxed_slice()); //alocate
    }

    fn render(&self, &mut self.local_raycast_coordinates) {
        const V: Vec3D = Vec3D::y(); 
        // being V a the canonical vector in y direction
        // being dir the vector that points towards the center of the rendering plane
        // if v and dir form basis, then ortho_v is the orthonormalized version of that basis  
        let ortho_v =  (V - V.dot(&self.dir) / self.dir.norm_squared() * self.dir ).normalize();
        let ortho_b = ortho_v.cross(&self.dir).normalize(); //R3 cross product is enough to get the third basis
        
        let half_width = render_size.0 / 2;
        let half_height = render_size.1 / 2;

        for h in -half_height..half_height {
            for w in -half_width..half_width{
                *self.local_raycast_coordinates[self.render_size.0 * h ] = 
            }
        }
    }
}
