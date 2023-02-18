//use web_sys::HtmlCanvasElement;

use web_sys::{WebGlBuffer, WebGlRenderingContext};


pub struct Geometry {
    pub coords: Vec<f32>,
    pub buffer: WebGlBuffer,
    pub attribute: i32,
    //kind: u32
}

impl Geometry {

    pub fn new(x: f32, y: f32, size: f32, buffer: WebGlBuffer, attribute: i32) -> Self {        
        let x2 = x + size;
        let y2 = y - size;  
         

        Self {
            buffer,
            attribute,
            coords: vec![
                x, y,
                x, y2,
                x2, y,
                x2, y,
                x, y2,
                x2, y2
            ],
        }
    }

    pub fn translation_mat(tx: f32, ty: f32) -> [f32; 9] {
        let translation_mat = [
            1., 0., 0.,
            0., 1., 0.,
            tx, ty, 1.
        ];

        translation_mat
    }

    pub fn scaling_mat(sx: f32, sy: f32) -> [f32; 9] {
        let scale_mat = [
            sx, 0., 0.,
            0., sy, 0.,
            0., 0., 1.
        ];

        scale_mat
    }

    pub fn rotation_mat(angle: f64) -> [f32; 9] {
        let cos = angle.cos() as f32;
        let sin = angle.sin() as f32;
        
        let rotation_mat = [
            cos, -sin, 0.,
            sin, cos, 0.,
            0., 0., 1.
        ];

        rotation_mat
    }

    pub fn translation_mat_3D(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
        let translation_mat = [
            1.,  0.,  0.,  0.,
            0.,  1.,  0.,  0.,
            0.,  0.,  1.,  0.,
            tx, ty, tz, 1.,
         ];

        translation_mat
    }

    pub fn scaling_mat_3D(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
        let scale_mat = [
            sx, 0.,  0.,  0.,
            0., sy,  0.,  0.,
            0.,  0., sz,  0.,
            0.,  0.,  0.,  1.,
          ];

        scale_mat
    }

    pub fn x_rotation_mat_3D(angle_in_radians: f64) -> [f32; 16] { // convert to radians
        let cos = angle_in_radians.cos() as f32;
        let sin = angle_in_radians.sin() as f32;
        
        let rotation_mat = [
            1., 0., 0., 0.,
            0., cos, sin, 0.,
            0., -sin, cos, 0.,
            0., 0., 0., 1.,
          ];

        rotation_mat
    }

    pub fn y_rotation_mat_3D(angle_in_radians: f64) -> [f32; 16] { // convert to radians
        let cos = angle_in_radians.cos() as f32;
        let sin = angle_in_radians.sin() as f32;
        
        let rotation_mat = [
            cos, 0., -sin, 0.,
            0., 1., 0., 0.,
            sin, 0., cos, 0.,
            0., 0., 0., 1.,
          ];

        rotation_mat
    }

    pub fn z_rotation_mat_3D(angle_in_radians: f64) -> [f32; 16] { // convert to radians
        let cos = angle_in_radians.cos() as f32;
        let sin = angle_in_radians.sin() as f32;
        
        let rotation_mat = [
            cos, sin, 0., 0.,
           -sin, cos, 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
         ];

        rotation_mat
    }
}

    

    