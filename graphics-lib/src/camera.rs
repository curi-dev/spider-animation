use std::{rc::Rc, cell::RefCell};
use nalgebra::Vector3;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;

use crate::{log, m4, webgl_utils::deg_to_rad};


pub struct Camera {
    eye: [f32; 3],
    acc_x_translation: Rc<RefCell<f32>>,
    acc_y_translation: Rc<RefCell<f32>>,
    acc_z_translation: Rc<RefCell<f32>>,
}

impl Camera {

    pub fn new(u: f32, v: f32, n: f32, canvas: &HtmlCanvasElement) -> Self {

        let x_translation = Rc::new(RefCell::new(0.));   
        let x_translation_clone = x_translation.clone();

        let y_translation = Rc::new(RefCell::new(0.));   
        let y_translation_clone = y_translation.clone();
    
        let z_translation = Rc::new(RefCell::new(0.));   
        let z_translation_clone = z_translation.clone();

        let closure = Closure::wrap(Box::new(move // move to events module (?)
            |event: web_sys::KeyboardEvent| {

            let key_code = event.key_code();         
            log(&format!("[KEY_CODE] {}", key_code));
                        
            if key_code == 38 {
                let updated_y_translation = *y_translation.borrow_mut() + 3.;

                *y_translation.borrow_mut() = updated_y_translation;
            }
   
            if key_code == 40 {
                let updated_y_translation = *y_translation.borrow_mut() - 3.;

                *y_translation.borrow_mut() = updated_y_translation;
            }

            if key_code == 39 {  
                let updated_x_translation = *x_translation.borrow_mut() + 3.;

                *x_translation.borrow_mut() = updated_x_translation;
            }
   
            if key_code == 37 {                 
                let updated_x_translation = *x_translation.borrow_mut() - 3.;

                *x_translation.borrow_mut() = updated_x_translation;
            }

            if key_code == 32 {                 
                let updated_z_translation = *z_translation.borrow_mut() + 3.;

                *z_translation.borrow_mut() = updated_z_translation;
            }

            if key_code == 13 {                 
                let updated_z_translation = *z_translation.borrow_mut() - 3.;

                *z_translation.borrow_mut() = updated_z_translation;
            }

        }) as Box<dyn FnMut(_)>);
    
        canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap(); 
        closure.forget();

        *x_translation_clone.borrow_mut() += u;
        *y_translation_clone.borrow_mut() += v;
        *z_translation_clone.borrow_mut() += n;

        Self {
            eye: [u, v, n], // x, y, z
            acc_x_translation: x_translation_clone,
            acc_y_translation: y_translation_clone,
            acc_z_translation: z_translation_clone,
        }
    }

    pub fn update_eye(&mut self) { // calling every frame sometimes no need
        self.eye[0] = *self.acc_x_translation.borrow(); 
        self.eye[1] = *self.acc_y_translation.borrow();
        self.eye[2] = *self.acc_z_translation.borrow();

    }

    pub fn curr_updated_vertex_position(&self) -> [f32; 3] {
        let eye_curr_position = self.curr_updated_position_matrix();
        
        [
            eye_curr_position[12],
            eye_curr_position[13],
            eye_curr_position[14]
        ]
    }

    fn curr_updated_position_matrix(&self) -> [f32; 16] {
        
        let mut updated_position = m4::M4::multiply_mat(
            m4::M4::identity(), 
            m4::M4::translation(
                self.eye[0], 
                self.eye[1], 
                self.eye[2],                
            )           
        );

        updated_position = m4::M4::multiply_mat(
            updated_position, 
            m4::M4::z_rotation( deg_to_rad( -30. ).into() )
        );

        updated_position
    }

    pub fn look_at(
        &self, 
        target: Vector3<f32>, 
        up: Vector3<f32>) -> [f32; 16] {
        
        let eye_curr_position = self.curr_updated_position_matrix();
        
        let camera_curr_position_vec = nalgebra::Vector3::new(
            eye_curr_position[12],
            eye_curr_position[13],
            eye_curr_position[14]
        );

        let z_axis = ( camera_curr_position_vec - target ).normalize();
        let x_axis = up.cross(&z_axis).normalize();
        let y_axis = z_axis.cross(&x_axis).normalize();

        return [
            x_axis[0], x_axis[1], x_axis[2], 0.,
            y_axis[0], y_axis[1], y_axis[2], 0.,
            z_axis[0], z_axis[1], z_axis[2], 0.,
            camera_curr_position_vec[0], camera_curr_position_vec[1], camera_curr_position_vec[2], 1.
        ]
    }
}