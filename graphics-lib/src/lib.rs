use std::panic;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlImageElement;
use web_sys::WebGlRenderingContext as Gl;

mod modules; // mod modules? not good

mod shaders;
mod setup;
mod programs;
mod webgl_utils;
mod setup_ui_control;
mod spider;
mod data_structures;
mod constants;
mod leg;
mod gpu_interface;
mod matrix_stack;

use setup::initialize_webgl_context;
use modules::m4::m4::M4 as m4;
use webgl_utils::resize_canvas_to_display_size;
use programs::base::ProgramBuilder;
use setup_ui_control::SetupUiControl;

use crate::constants::*;
use crate::gpu_interface::GpuInterface;
use crate::programs::base::Program;

use crate::spider::Spider; // separate types and data structures 
use crate::webgl_utils::deg_to_rad;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct GraphicsClient {
    //gl: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    // a_positions: i32,
    // u_matrix: WebGlUniformLocation,
    // a_color: i32,
    ui_control: SetupUiControl,
    spider: spider::Spider,
    gpu_interface: GpuInterface
}

unsafe impl Send for GraphicsClient {}

#[wasm_bindgen]
impl GraphicsClient {

    #[wasm_bindgen(constructor)]
    pub fn new(_image: HtmlImageElement) -> Self {    
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let (gl, canvas, ) = initialize_webgl_context().unwrap(); 
        
        // create and compile shaders & link the program with these shaders (can pass the shaders here [dependency inversion]) 
        let program = ProgramBuilder::build(&gl, Program::Program3); // refactor!

        gl.use_program(Some(&program));

        let ui_control = SetupUiControl::new(&canvas);

        let gpu_interface = GpuInterface::new(gl, &program);

        Self {
            spider: Spider::new(&canvas),
            canvas,
            gpu_interface,
            ui_control,
        }      
    }


    pub fn render(&mut self) {          
        //deltatime = deltatime / 1000.;
        self.gpu_interface.gl.clear(Gl::COLOR_BUFFER_BIT);
        resize_canvas_to_display_size(&self.gpu_interface.gl, &self.canvas);
        
        let aspect = (self.canvas.client_width() / self.canvas.client_height()) as f32;
        //let aspect = ( self.canvas.client_height() / self.canvas.client_width() ) as f32;

        let ui_rotation_x_body = self.ui_control.acc_x_rotation_body.try_borrow().unwrap();
        let ui_rotation_y_body = self.ui_control.acc_y_rotation_body.try_borrow().unwrap();
        let ui_translate_z_body = self.ui_control.acc_z_translation_body.try_borrow().unwrap();
    
        // creating the model matrix for the body
        let mut body_model_matrix = m4::perspective(
            deg_to_rad(DEFAULT_FIELD_OF_VIEW_IN_RADIANS),
            aspect,
            DEFAULT_Z_NEAR,
            DEFAULT_Z_FAR
        );

        body_model_matrix = m4::translate_3_d(body_model_matrix, m4::translation(
            INITIAL_BODY_DISPLACEMENT_X, 
            INITIAL_BODY_DISPLACEMENT_Y,
            INITIAL_BODY_DISPLACEMENT_Z, 
        ));

        // transformations coming from ui control - 3 transformations
        body_model_matrix = m4::translate_3_d(body_model_matrix, m4::translation(
            0., 
            0., 
            *ui_translate_z_body 
        ));

        body_model_matrix = m4::x_rotate_3_d( // is that better to put it inside animate() ?
            body_model_matrix,
            m4::x_rotation(deg_to_rad(*ui_rotation_x_body).into())
        );

        body_model_matrix = m4::y_rotate_3_d( // is that better to put it inside animate() ?
            body_model_matrix,
            m4::y_rotation(deg_to_rad(*ui_rotation_y_body).into())
        );    

        // when using rust and webgl together, rust is used for data manipulation and
        // calculation, while webgl is used for rendering and displaying the data on the
        // client-side. this buffer is not being persisted on rust's layout memory, but rather
        // in the graphics card's memory on the client-side
        let positions_buffer = self.gpu_interface.gl.create_buffer().unwrap();  
        let colors_buffer = self.gpu_interface.gl.create_buffer().unwrap(); 
        

        // self.send_positions_to_gpu(&self.spider.head_data, &positions_buffer);
        // self.send_colors_to_gpu(&self.spider.body_colors, &colors_buffer);
        
        // let head_model_matrix = m4::translate_3_d(body_model_matrix, m4::translation(
        //     BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 
        //     3., 
        //     BODY_DEPTH / 2. - HEAD_DEPTH / 2. 
        // ));

        // self.consume_data(
        //     self.spider.head_data.len() as i32 / 3, 
        //     Gl::TRIANGLES, 
        //     &head_model_matrix
        // );  

        self.spider.animate_body(
            &self.gpu_interface, 
            &body_model_matrix, 
            &positions_buffer, 
            &colors_buffer);

        self.spider.animate_front_legs(
            &self.gpu_interface, 
            &body_model_matrix, 
            &positions_buffer, 
            &colors_buffer
        );

        //self.spider.animate_back_legs(&self.gpu_interface, &body_model_matrix, &positions_buffer, &colors_buffer);
        //self.spider.animate_middle_legs(&self.gpu_interface, &body_model_matrix, &positions_buffer, &colors_buffer)
    
    }
}






