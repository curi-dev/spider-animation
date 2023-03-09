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

}

impl SetupUiControl {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        // let active_object = Rc::new(RefCell::new(0));   
        // let active_object_clone = active_object.clone();

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

        let keydown_closure = Closure::wrap(Box::new(move // move to events module (?)
            |event: web_sys::KeyboardEvent| {

            let key_code = event.key_code();         
            log(&format!("[KEY_CODE] {}", key_code));
                        
            if key_code == 38 {
                let x_rotation_angle = *x_rotation_body.borrow_mut();

                log(&format!("[MORE] y_rotation_angle: {} ", x_rotation_angle));

                *x_rotation_body.borrow_mut() = x_rotation_angle + 3.5;
            }

    
            if key_code == 40 {
                let x_rotation_angle = *x_rotation_body.borrow_mut();

                log(&format!("[MORE] x_rotation_angle: {} ", x_rotation_angle));
    
                *x_rotation_body.borrow_mut() = x_rotation_angle - 3.5;
            }


            if key_code == 39 {  
                let y_rotation_angle = *y_rotation_body.borrow_mut();

                log(&format!("[MORE] y_rotation_angle: {} ", y_rotation_angle));

                *y_rotation_body.borrow_mut() = y_rotation_angle + 3.5;
            }

    
            if key_code == 37 {                 
                let y_rotation_angle = *y_rotation_body.borrow_mut();

                log(&format!("[MINUS] y_rotation_angle: {} ", y_rotation_angle));
            
                *y_rotation_body.borrow_mut() = y_rotation_angle - 3.5;
            }


            if key_code == 32 {
                let z_translation_inner = *z_translation_body.borrow_mut();
            
                log(&format!("[MINUS] z_translation: {} ", z_translation_inner));

                *z_translation_body.borrow_mut() = z_translation_inner - 5.;
            }
        

            if key_code == 13 {  
                let z_translation_inner = *z_translation_body.borrow_mut();
            
                log(&format!("[MINUS] z_translation: {} ", z_translation_inner));
    
                *z_translation_body.borrow_mut() = z_translation_inner + 5.;
        
            }
        }) as Box<dyn FnMut(_)>);
    
        canvas.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap(); // ?
        keydown_closure.forget();
           
        Self {
            is_active: true,
            acc_z_translation: z_translation_clone,
            acc_x_rotation: x_rotation_clone,
            acc_y_rotation: y_rotation_clone,
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

pub enum Move {
    Forward,
    Left,
    Right,
    Jump,
    Static
}
pub struct SpiderControl{
    pub is_active: bool,
    pub direction: Rc<RefCell<Move>>,
    pub go_forward: Rc<RefCell<bool>>,    
    pub go_back: Rc<RefCell<bool>>,    
    pub turn_left: Rc<RefCell<bool>>,    
    pub turn_right: Rc<RefCell<bool>>,    
}

impl SpiderControl {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let go_forward = Rc::new(RefCell::new(false));   
        let go_forward_clone = go_forward.clone();

        let go_back = Rc::new(RefCell::new(false));   
        let go_back_clone = go_back.clone();

        let turn_left = Rc::new(RefCell::new(false));   
        let turn_left_clone = turn_left.clone();

        let turn_right = Rc::new(RefCell::new(false));   
        let turn_right_clone = turn_right.clone();

        let direction = Rc::new(RefCell::new(Move::Static));   
        let direction_clone = direction.clone();
    
        let keydown_closure = Closure::wrap(Box::new(move // move to events module (?)
            |event: web_sys::KeyboardEvent| {

            let key_code = event.key_code();         
            log(&format!("[KEY_CODE] {}", key_code));
                        
            // spider control
            if key_code == 65 {
                // turn left
                *turn_left.borrow_mut() = true;
                *direction.borrow_mut() = Move::Left;
            }

            if key_code == 68 {
                // turn right
                *turn_right.borrow_mut() = true;
                *direction.borrow_mut() = Move::Right;
            }

            if key_code == 87 {
                // go forward
                *go_forward.borrow_mut() = true;
                *direction.borrow_mut() = Move::Forward;
            }

            if key_code == 83 {
                // go back
                *go_back.borrow_mut() = true;
            }
        }) as Box<dyn FnMut(_)>);

        // let keyup_closure = Closure::wrap(Box::new(move // move to events module (?)
        //     |event: web_sys::KeyboardEvent| {

        //     let key_code = event.key_code();         
        //     log(&format!("[KEY_CODE] {}", key_code));
                        
        //     // spider control
        //     if key_code == 65 {
        //         // turn left
        //         //*turn_left.borrow_mut() = false;
        //     }

        //     if key_code == 68 {
        //         // turn right
        //         //*turn_right.borrow_mut() = false;
        //     }

        //     if key_code == 87 {
        //         // go forward
        //         //*go_forward.borrow_mut() = false;
        //     }

        //     if key_code == 83 {
        //         // go back
        //         //*go_back.borrow_mut() = false;
        //     }
        // }) as Box<dyn FnMut(_)>);
    
        canvas.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap(); // ?
        //canvas.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref()).unwrap(); // ?
        keydown_closure.forget();
        //keyup_closure.forget();
           
        Self {
            direction: direction_clone,
            go_forward: go_forward_clone,
            go_back: go_back_clone,
            turn_left: turn_left_clone,
            turn_right: turn_right_clone,
            is_active: true,
        }
    }

    fn disable_control(&mut self) {
        self.is_active = false;
    }

    fn enable_control(&mut self) {
        self.is_active = true;
    }

}
