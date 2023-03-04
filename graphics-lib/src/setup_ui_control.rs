use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;

use crate::log;


pub struct SetupUiControl{
    pub is_active: bool,
    pub acc_z_translation: Rc<RefCell<f32>>, 
    pub acc_x_rotation: Rc<RefCell<f32>>, 
    pub acc_y_rotation: Rc<RefCell<f32>>,
    pub acc_z_translation_body: Rc<RefCell<f32>>, 
    pub acc_x_rotation_body: Rc<RefCell<f32>>, 
    pub acc_y_rotation_body: Rc<RefCell<f32>>,
    //pub active_object: Rc<RefCell<u8>>
    pub active_object: u8
}

impl SetupUiControl {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let active_object = Rc::new(RefCell::new(0));   
        let active_object_clone = active_object.clone();

        let z_translation = Rc::new(RefCell::new(0.));   
        let z_translation_clone = z_translation.clone();
    
        let x_rotation = Rc::new(RefCell::new(0.));   
        let x_rotation_clone = x_rotation.clone();
    
        let y_rotation = Rc::new(RefCell::new(0.));   
        let y_rotation_clone = y_rotation.clone();

        let z_translation_body = Rc::new(RefCell::new(0.));   
        let z_translation_body_clone = z_translation_body.clone();
    
        let x_rotation_body = Rc::new(RefCell::new(0.));   
        let x_rotation_body_clone = x_rotation_body.clone();
    
        let y_rotation_body = Rc::new(RefCell::new(0.));   
        let y_rotation_body_clone = y_rotation_body.clone();
    
        let closure = Closure::wrap(Box::new(move // move to events module (?)
            |event: web_sys::KeyboardEvent| {

            let key_code = event.key_code();         
            log(&format!("[KEY_CODE] {}", key_code));
            
            //let mut active_object_handler = active_object.borrow_mut();
            //log(&format!("[OBJECT] active_object: {} ", *active_object_handler));
            
            if key_code == 38 {
                // if *active_object_handler == 0 {
                //     let x_rotation_angle = *x_rotation.borrow_mut();
    
                //     log(&format!("[MORE] y_rotation_angle: {} ", x_rotation_angle));
    
                //     *x_rotation.borrow_mut() = x_rotation_angle + 3.5;
                // } else {
                    let x_rotation_angle = *x_rotation_body.borrow_mut();
    
                    log(&format!("[MORE] y_rotation_angle: {} ", x_rotation_angle));
    
                    *x_rotation_body.borrow_mut() = x_rotation_angle + 3.5;
                //}             
            }

    
            if key_code == 40 {
                // if *active_object_handler == 0 {
                //     let x_rotation_angle = *x_rotation.borrow_mut();
    
                //     log(&format!("[MORE] x_rotation_angle: {} ", x_rotation_angle));
        
                //     *x_rotation.borrow_mut() = x_rotation_angle - 3.5;
                // } else {
                    let x_rotation_angle = *x_rotation_body.borrow_mut();
    
                    log(&format!("[MORE] x_rotation_angle: {} ", x_rotation_angle));
        
                    *x_rotation_body.borrow_mut() = x_rotation_angle - 3.5;
                //}
                
            }


            if key_code == 39 {  
                //log(&format!("[OBJECT] active_object: {} ", *active_object_handler));
                
                // if *active_object_handler == 0 {
                //     let y_rotation_angle = *y_rotation.borrow_mut();
    
                //     log(&format!("[MORE] y_rotation_angle: {} ", y_rotation_angle));
        
                //     *y_rotation.borrow_mut() = y_rotation_angle + 3.5;
                // } else {
                    let y_rotation_angle = *y_rotation_body.borrow_mut();
    
                    log(&format!("[MORE] y_rotation_angle: {} ", y_rotation_angle));
    
                    *y_rotation_body.borrow_mut() = y_rotation_angle + 3.5;
                //}
            }

    
            if key_code == 37 {                 
                // if *active_object_handler == 0 {
                //     let y_rotation_angle = *y_rotation.borrow_mut();
    
                //     log(&format!("[MINUS] y_rotation_angle: {} ", y_rotation_angle));
                    
                //     *y_rotation.borrow_mut() = y_rotation_angle - 3.5;
                // } else {
                    let y_rotation_angle = *y_rotation_body.borrow_mut();
    
                    log(&format!("[MINUS] y_rotation_angle: {} ", y_rotation_angle));
                
                    *y_rotation_body.borrow_mut() = y_rotation_angle - 3.5;
                //}                
            }


            if key_code == 65 {
                // if *active_object_handler == 0 {
                //     let z_translation_inner = *z_translation.borrow_mut();
                
                //     log(&format!("[MINUS] z_translation: {} ", z_translation_inner));
        
                //     *z_translation.borrow_mut() = z_translation_inner + 5.;
                // } else {
                    let z_translation_inner = *z_translation_body.borrow_mut();
                
                    log(&format!("[MINUS] z_translation: {} ", z_translation_inner));
        
                    *z_translation_body.borrow_mut() = z_translation_inner + 5.;
                //}
            }


            if key_code == 32 {
                // if *active_object_handler == 0 {
                //     let z_translation_inner = *z_translation.borrow_mut();
                
                //     log(&format!("[MINUS] z_translation: {} ", z_translation_inner));
    
                //     *z_translation.borrow_mut() = z_translation_inner - 5.;
                // } else {
                    let z_translation_inner = *z_translation_body.borrow_mut();
                
                    log(&format!("[MINUS] z_translation: {} ", z_translation_inner));
    
                    *z_translation_body.borrow_mut() = z_translation_inner - 5.;
                //}               
            }


            if key_code == 13 {  
                //let mut active_object_handler = active_object.borrow_mut();
                
                //log(&format!("[OBJECT] active_object: {} ", *active_object_handler));
                
                // if *active_object_handler == 0 {
                //     *active_object_handler = 1;
                // } else {
                //     *active_object_handler = 0;
                // }
            }
        }) as Box<dyn FnMut(_)>);
    
        canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap(); // ?
        closure.forget();
        
           
        Self {
            is_active: true,
            acc_z_translation: z_translation_clone,
            acc_x_rotation: x_rotation_clone,
            acc_y_rotation: y_rotation_clone,
            active_object: 1,
            acc_z_translation_body: z_translation_body_clone,
            acc_x_rotation_body: x_rotation_body_clone,
            acc_y_rotation_body: y_rotation_body_clone,
        }
    }

    fn disable_control(&mut self) {
        self.is_active = false;
    }

    fn enable_control(&mut self) {
        self.is_active = true;
    }

}

// create some type of relationship to make more clear
enum KeyboardKeys {
    Space(u8),
    Enter(u8),
    DirectionalR(u8),
    DirectionalL(u8),
    DirectionalD(u8),
    DirectionalU(u8),
    A(u8) 
}

