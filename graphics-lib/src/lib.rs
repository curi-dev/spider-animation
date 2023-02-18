use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use console_error_panic_hook;
use js_sys::WebAssembly;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGlRenderingContext, HtmlCanvasElement, WebGlUniformLocation, HtmlImageElement};
use web_sys::WebGlRenderingContext as Gl;

mod modules; // mod modules? not good

mod shaders;
mod setup;
mod programs;
mod webgl_utils;
mod setup_ui_control;
mod spider;

use setup::initialize_webgl_context;
use modules::m4::m4::M4 as m4;
use webgl_utils::resize_canvas_to_display_size;
use programs::base::ProgramBuilder;
use setup_ui_control::SetupUiControl;

use crate::programs::base::Program;
use crate::spider::{Spider, LegsDirection}; // separate types and data structures 
use crate::webgl_utils::deg_to_rad;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct GraphicsClient {
    gl: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    a_positions: i32,
    u_matrix: WebGlUniformLocation,
    a_color: i32,
    x_rotation_ptr: Rc<RefCell<f32>>,
    y_rotation_ptr: Rc<RefCell<f32>>,
    z_rotation_acc: f32,
    ui_control: SetupUiControl,
    speed: f32, // change this name
    spider: spider::Spider
}

unsafe impl Send for GraphicsClient {}

#[wasm_bindgen]
impl GraphicsClient {

    #[wasm_bindgen(constructor)]
    pub fn new(_image: HtmlImageElement) -> Self {    
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let (gl, canvas) = initialize_webgl_context().unwrap();

        // create and compile shaders & link the program with these shaders (can pass the shaders here [dependency inversion]) 
        let program = ProgramBuilder::build(&gl, Program::Program3); // refactor!

        let a_positions_loc = gl.get_attrib_location(&program, "aPosition");
        let u_matrix_loc = gl.get_uniform_location(&program, "uMatrix").unwrap();
        let a_color_loc = gl.get_attrib_location(&program, "aColor");

        gl.use_program(Some(&program));

        let x_rotation = Rc::new(RefCell::new(0. as f32));   
        let x_rotation_ptr = x_rotation.clone();

        let y_rotation = Rc::new(RefCell::new(0. as f32));   
        let y_rotation_ptr = y_rotation.clone();

        let z_rotation = Rc::new(RefCell::new(0. as f32));   
        let z_rotation_ptr = z_rotation.clone();

        // NOW I HAVE THE TWO POINTERS MODIFYING THE VALUE (?)
        // let closure = Closure::wrap(Box::new(move // move to events module (?)
        //     |event: web_sys::KeyboardEvent| {
            
        //     let key_code = event.key_code();
        //     log(&format!("keycode: {:?} ", key_code));

        //     if key_code == 39 {
        //         let x_rotation_angle = *x_rotation.borrow_mut();

        //         log(&format!("[MORE] x_rotation_angle: {} ", x_rotation_angle));

        //         *x_rotation.borrow_mut() = x_rotation_angle + 5.;
        //     }

        //     if key_code == 37 {
        //         let z_rotation_angle = *z_rotation.borrow_mut();

        //         log(&format!("[MINUS] x_rotation_angle: {} ", z_rotation_angle));
                
        //         *z_rotation.borrow_mut() = z_rotation_angle - 5.;
        //     }

        //     if key_code == 38 {
        //         let y_rotation_angle = *y_rotation.borrow_mut();
                
        //         *y_rotation.borrow_mut() = y_rotation_angle + 5.;
        //     }

        //     if key_code == 40 {
        //         let y_rotation_angle = *y_rotation.borrow_mut();
                
        //         *y_rotation.borrow_mut() = y_rotation_angle - 5.;
        //     }
        // }) as Box<dyn FnMut(_)>);

        // canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap(); // ?
        // closure.forget();

        let ui_control = SetupUiControl::new();

        Self {
            canvas,gl,
            a_positions:a_positions_loc,
            u_matrix:u_matrix_loc,
            a_color:a_color_loc, 
            z_rotation_acc: 0.,
            speed: 10., 
            x_rotation_ptr, 
            y_rotation_ptr, 
            ui_control,
            spider: Spider::new()
        }      
    }


    pub fn render(&mut self) {          
        self.gl.clear(Gl::COLOR_BUFFER_BIT);
        
        //deltatime = deltatime / 1000.;
       
        resize_canvas_to_display_size(&self.gl, &self.canvas);

        let positions_buffer = self.gl.create_buffer().unwrap(); // it leaves inside rust? (try to transfer into the function)
        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&positions_buffer));

        self.gl.vertex_attrib_pointer_with_i32(
            self.a_positions as u32, 
            3, 
            Gl::FLOAT, 
            false, 
            0, 
            0,
        );

        self.send_positions_to_gpu(&self.spider.upper_and_middle_legs_data);

        let mut matrix = m4::projection(self.canvas.client_width() as f32, self.canvas.client_height() as f32, 600.);
        matrix = m4::translate_3_d(matrix, m4::translation(300., 200., 199.));
        matrix = m4::scale_3_d(matrix, m4::scaling(6., 4.,1.));

        let mut x_rotation_angle: f32 = 0.;
        let mut y_rotation_angle: f32 = 0.;
        //let mut z_rotation_angle: f32 = 0.;

        match self.ui_control.is_active {
            true => {
                x_rotation_angle = *self.x_rotation_ptr.try_borrow().unwrap();
                y_rotation_angle = *self.y_rotation_ptr.try_borrow().unwrap();
                //z_rotation_angle = *self.z_rotation_ptr.try_borrow().unwrap();
            },
            false => {
                let deltatime = 0.026;
                let mut displacement: f32 = self.speed * deltatime; // angle_displacement
                               
                if self.spider.legs_direction == LegsDirection::Back {
                    displacement *= -1.;
                }
                
                self.z_rotation_acc += displacement;

                if !self.spider.move_range.contains(&self.z_rotation_acc) {
                    self.spider.change_direction();
                }
            },   
        }

        matrix = m4::x_rotate_3_d(
            matrix,
            m4::x_rotation(deg_to_rad(x_rotation_angle).into())
        );
        
        matrix = m4::y_rotate_3_d(
            matrix,
            m4::y_rotation(deg_to_rad(y_rotation_angle).into())
        );
    
        matrix = m4::z_rotate_3_d(
            matrix,
            m4::z_rotation(deg_to_rad(self.z_rotation_acc).into())
        );                

        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &matrix);

        let colors_buffer = self.gl.create_buffer().unwrap(); // it leaves inside rust? (try to transfer into the function)
        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&colors_buffer));
        
        self.gl.vertex_attrib_pointer_with_i32(
            self.a_color as u32, 
            3, 
            Gl::UNSIGNED_BYTE, 
            true, 
            0, 
            0,
        );

        self.send_colors_to_gpu(&self.spider.colors);
        
        self.consume_data(self.spider.upper_and_middle_legs_data.len() as i32 / 3, Gl::TRIANGLES);
        
        // edit the transformation matrix for the second part of the pawn but using the same base of coordinates
        matrix = m4::projection(self.canvas.client_width() as f32, self.canvas.client_height() as f32, 600.);
        matrix = m4::translate_3_d(matrix, m4::translation(500., 200., 199.));
        matrix = m4::scale_3_d(matrix, m4::scaling(4., 2.,1.));

        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &matrix);

        self.consume_data(self.spider.upper_and_middle_legs_data.len() as i32 / 3, Gl::TRIANGLES);
        
        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&positions_buffer));

        self.gl.vertex_attrib_pointer_with_i32(
            self.a_positions as u32, 
            3, 
            Gl::FLOAT, 
            false, 
            0, 
            0,
        );

        self.send_positions_to_gpu(&self.spider.bottom_legs_data);
        
        matrix = m4::projection(self.canvas.client_width() as f32, self.canvas.client_height() as f32, 600.); // projection matrix is always the same
        matrix = m4::translate_3_d(matrix, m4::translation(700., 200., 199.));
        matrix = m4::scale_3_d(matrix, m4::scaling(3.75, 0.75,1.));

        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &matrix);

        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&colors_buffer));
        
        self.gl.vertex_attrib_pointer_with_i32(
            self.a_color as u32, 
            3, 
            Gl::UNSIGNED_BYTE, 
            true, 
            0, 
            0,
        );
        self.send_colors_to_gpu(&self.spider.colors);

        self.consume_data(self.spider.bottom_legs_data.len() as i32 / 3, Gl::TRIANGLES);
       
    }

    fn send_positions_to_gpu(&self, positions: &[f32]) {
        let memory_buffer_view = wasm_bindgen::memory() // persist this memory buffer?
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

        let ptr_mem_loc = positions.as_ptr() as u32 / 4; // 4 bytes each
        let coords_ptr = js_sys::Float32Array::new(&memory_buffer_view)
            .subarray(ptr_mem_loc, ptr_mem_loc + positions.len() as u32);
        self.gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &coords_ptr, Gl::DYNAMIC_DRAW);
        self.gl.enable_vertex_attrib_array(self.a_color as u32);
    }

    fn send_colors_to_gpu(&self, colors: &[u8]) {
        let memory_buffer_view = wasm_bindgen::memory() // persist this memory buffer?
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

        let ptr_mem_loc = colors.as_ptr() as u32; // 4 bytes each
        let coords_ptr = js_sys::Uint8Array::new(&memory_buffer_view)
            .subarray(ptr_mem_loc, ptr_mem_loc + colors.len() as u32);
        self.gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &coords_ptr, Gl::STATIC_DRAW);
        self.gl.enable_vertex_attrib_array(self.a_positions as u32);
    }
    
    fn consume_data(&self, vert_count: i32, mode: u32) {
        self.gl.draw_arrays(mode, 0, vert_count);
    }

    fn get_center_of_canvas(&self) -> (f32, f32) {
        let center_x = (self.canvas.client_width() / 2) as f32;
        let center_y = (self.canvas.client_height() / 2) as f32;

        (center_x, center_y)
    }
}






