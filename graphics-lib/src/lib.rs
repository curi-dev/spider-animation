use std::panic;
use camera::Camera;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlImageElement;
use web_sys::WebGlRenderingContext as Gl;

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
mod camera;
mod m4;

use crate::constants::*;
use crate::data_structures::*;
use crate::gpu_interface::GpuInterface;
use crate::programs::base::Program;
use crate::spider::Spider; 

use webgl_utils::{resize_canvas_to_display_size, deg_to_rad};
use setup::initialize_webgl_context;
use programs::base::ProgramBuilder;
use m4::M4;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct GraphicsClient {
    canvas: HtmlCanvasElement,
    camera: Camera,
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

        let gpu_interface = GpuInterface::new(gl, &program);

        let camera = Camera::new(0., 0., 100., &canvas);

        Self {
            spider:Spider::new(&canvas),
            canvas,
            gpu_interface, 
            camera 
        }      
    }


    pub fn render(&mut self) {          
        //deltatime = deltatime / 1000.;
        self.gpu_interface.gl.clear(Gl::COLOR_BUFFER_BIT);
        resize_canvas_to_display_size(&self.gpu_interface.gl, &self.canvas);
        
        let aspect = (self.canvas.client_width() / self.canvas.client_height()) as f32;

        // setup camera properly
        self.camera.update_eye();
        
        let look_at = self.camera.look_at(
            nalgebra::Vector3::new(0., 0., 0.), 
            nalgebra::Vector3::new(0., 1., 0.)
        );

        let view_mat = nalgebra::Matrix4::from_column_slice(
            &look_at
            )
            .try_inverse()
            .unwrap();

        let projection_mat = nalgebra::Perspective3::new(
            aspect, 
            deg_to_rad(DEFAULT_FIELD_OF_VIEW_IN_RADIANS), 
            DEFAULT_Z_NEAR, 
            DEFAULT_Z_FAR
        );
        let view_projection_mat = M4::multiply_mat(
            projection_mat
                .to_homogeneous()
                .as_slice()
                .try_into()
                .unwrap(), 
            view_mat
                //.to_homogeneous() // complete the mat?
                .as_slice()
                .try_into()
                .unwrap()
        );

        // when using rust and webgl together, rust is used for data manipulation and
        // calculation, while webgl is used for rendering and displaying the data on the
        // client-side. this buffer is not being persisted on rust's layout memory, but rather
        // in the graphics card's memory on the client-side
        let positions_buffer = self.gpu_interface.gl.create_buffer().unwrap();  
        let colors_buffer = self.gpu_interface.gl.create_buffer().unwrap(); 
        
        // floor below
        let floor_data = get_floor_data();
        self.gpu_interface.send_positions_to_gpu(&floor_data, &positions_buffer);
        self.gpu_interface.send_colors_to_gpu(&self.spider.body_colors, &colors_buffer);

        self.gpu_interface.consume_data(
            floor_data.len() as i32 / 3, 
            Gl::TRIANGLES,
            &view_projection_mat
        );
        // floor above

        let mut body_model_matrix = M4::x_rotate_3_d(
            view_projection_mat,
            M4::y_rotation( deg_to_rad( -60. ).into() )
        );

        body_model_matrix = self.spider.animate_body(
            &body_model_matrix,
            &self.gpu_interface,  
            &positions_buffer, 
            &colors_buffer,
        );

        self.spider.animate_front_legs(
            &self.gpu_interface, 
            &body_model_matrix, 
            &positions_buffer, 
            &colors_buffer
        );

        self.spider.animate_back_legs(
            &self.gpu_interface, 
            &body_model_matrix, 
            &positions_buffer, 
            &colors_buffer
        );
        
        self.spider.animate_middle_legs(
            &self.gpu_interface, 
            &body_model_matrix, 
            &positions_buffer, 
            &colors_buffer
        );

        // head below
        self.gpu_interface.send_positions_to_gpu(&self.spider.head_data, &positions_buffer);
        self.gpu_interface.send_colors_to_gpu(&self.spider.body_colors, &colors_buffer);
        
        let head_model_matrix = M4::translate_3_d(
            body_model_matrix, 
            M4::translation(
            BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 
            3., 
            BODY_DEPTH / 2. - HEAD_DEPTH / 2. 
        ));

        self.gpu_interface.consume_data(
            self.spider.head_data.len() as i32 / 3, 
            Gl::TRIANGLES, 
            &head_model_matrix
        );
        // head above
    
    }
}






