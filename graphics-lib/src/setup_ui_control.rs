use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;

use crate::log;


pub struct SetupUiControl{
    pub is_active: bool,
    pub acc_z_translation: Rc<RefCell<f32>>, 
    pub acc_x_rotation: Rc<RefCell<f32>>, 
    pub acc_y_rotation: Rc<RefCell<f32>>
}

impl SetupUiControl {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
            // add events
        let z_translation = Rc::new(RefCell::new(0.));   
        let z_translation_clone = z_translation.clone();
    
        let x_rotation = Rc::new(RefCell::new(0.));   
        let x_rotation_clone = x_rotation.clone();
    
        let y_rotation = Rc::new(RefCell::new(0.));   
        let y_rotation_clone = y_rotation.clone();
    
        let closure = Closure::wrap(Box::new(move // move to events module (?)
            |event: web_sys::KeyboardEvent| {
            
            let key_code = event.key_code();
            
            log(&format!("[KEY_CODE] {}", key_code));
    
            if key_code == 39 {
                let y_rotation_angle = *y_rotation.borrow_mut();
    
                log(&format!("[MORE] y_rotation_angle: {} ", y_rotation_angle));
    
                *y_rotation.borrow_mut() = y_rotation_angle + 3.5;
            }
    
            if key_code == 37 {
                let y_rotation_angle = *y_rotation.borrow_mut();
    
                log(&format!("[MINUS] y_rotation_angle: {} ", y_rotation_angle));
                
                *y_rotation.borrow_mut() = y_rotation_angle - 3.5;
            }
    
            if key_code == 38 {
                let x_rotation_angle = *x_rotation.borrow_mut();
    
                log(&format!("[MORE] y_rotation_angle: {} ", x_rotation_angle));
    
                *x_rotation.borrow_mut() = x_rotation_angle + 3.5;
            }
    
            if key_code == 40 {
                let x_rotation_angle = *x_rotation.borrow_mut();
    
                log(&format!("[MORE] y_rotation_angle: {} ", x_rotation_angle));
    
                *x_rotation.borrow_mut() = x_rotation_angle - 3.5;
            }

            if key_code == 65 {
                let z_translation_inner = *z_translation.borrow_mut();
                
                log(&format!("[MINUS] z_translation: {} ", z_translation_inner));
    
                *z_translation.borrow_mut() = z_translation_inner + 5.;
            }

            if key_code == 32 {
                let z_translation_inner = *z_translation.borrow_mut();
                
                log(&format!("[MINUS] z_translation: {} ", z_translation_inner));
    
                *z_translation.borrow_mut() = z_translation_inner - 5.;
            }
        }) as Box<dyn FnMut(_)>);
    
        canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap(); // ?
        closure.forget();
        
           
        Self {
            is_active: true,
            acc_z_translation: z_translation_clone,
            acc_x_rotation: x_rotation_clone,
            acc_y_rotation: y_rotation_clone,
        }
    }

    fn disable_control(&mut self) {
        self.is_active = false;
    }

    fn enable_control(&mut self) {
        self.is_active = true;
    }

}