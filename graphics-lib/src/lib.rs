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

        let camera = Camera::new(35., 35., 100., &canvas);

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

        let spider_last_position = self.spider.last_pos_model_mat;
        let target = match spider_last_position {
            Some(position_mat) => nalgebra::Vector3::new(
                    position_mat[12], 
                    position_mat[13], 
                    position_mat[14]
                ),
            None => nalgebra::Vector3::new(0., 0., 0.),
        };
        
        let look_at = self.camera.look_at(
            target, 
            nalgebra::Vector3::new(0., 1., 0.)
        );

        let camera_matrix = nalgebra::Matrix4::from_column_slice(
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
        let camera_projection_matrix = M4::multiply_mat(
            projection_mat
                .to_homogeneous()
                .as_slice()
                .try_into()
                .unwrap(), 
            camera_matrix
                .as_slice()
                .try_into()
                .unwrap()
        );
  
        // when using rust and webgl together, rust is used for data manipulation and
        // calculation, while webgl is used for rendering and displaying the data on the
        // client-side. this buffer is not being persisted on rust's layout memory, but rather
        // in the graphics card's memory on the client-side
        
        let positions_buffer = self.gpu_interface.gl.create_buffer().unwrap();  
        let normals_buffer = self.gpu_interface.gl.create_buffer().unwrap();

        // directional light setup
        let light_position = [0., 200., 0.];
        
        // floor below (turn into entity?)
        let floor_data = get_floor_data();
        self.gpu_interface.send_positions_to_gpu(&floor_data, &positions_buffer);
        self.gpu_interface.send_normals_to_gpu(&get_floor_normals(), &normals_buffer);
        
        let r = 1. / 255. * 15.;
        let g = 1. / 255. * 45.;
        let b = 1. / 255. * 10.;
        self.gpu_interface.consume_data(
            floor_data.len() as i32 / 3, 
            Gl::TRIANGLES,
            &camera_projection_matrix,
            &m4::M4::identity(),
            &light_position, // singleton
            (r, g, b)
        );
        // floor above

        let body_model_matrix = self.spider.animate_body(
            &camera_projection_matrix, 
            &self.gpu_interface,  
            &positions_buffer, 
            &normals_buffer,
            &light_position // singleton
        );

        self.spider.animate_front_legs(
            &self.gpu_interface, 
            &body_model_matrix, // camera + projection + body move (translation and rotation)
            &positions_buffer, 
            &normals_buffer,
            &light_position // singleton
        );

        self.spider.animate_back_legs(
            &self.gpu_interface, 
            &body_model_matrix, 
            &positions_buffer, 
            &normals_buffer,
            &light_position // singleton
        );
        
        self.spider.animate_middle_legs(
            &self.gpu_interface, 
            &body_model_matrix, 
            &positions_buffer, 
            &normals_buffer,
            &light_position // singleton
        );

        // head below
        self.gpu_interface.send_positions_to_gpu(&self.spider.head_data, &positions_buffer);
        self.gpu_interface.send_normals_to_gpu(&get_body_normals(), &normals_buffer);
        //self.gpu_interface.send_colors_to_gpu(&self.spider.body_colors, &colors_buffer);
        
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
            &head_model_matrix, // D.R.Y
            &head_model_matrix, // D.R.Y
            &light_position,
            (0.08, 0.08, 0.08)
        );
        // head above

        // delete the buffers stored on gpu card [is that really necessary?]
        // store a pointer to the buffer on rust's memory (?)
        self.gpu_interface.gl.delete_buffer(Some(&positions_buffer));
        self.gpu_interface.gl.delete_buffer(Some(&normals_buffer));
    
    }
}






