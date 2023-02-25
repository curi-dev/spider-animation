use std::panic;
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
    gl: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    a_positions: i32,
    u_matrix: WebGlUniformLocation,
    a_color: i32,
    camera: SetupUiControl,
    spider: spider::Spider,
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

        let ui_control = SetupUiControl::new();

    
        Self {
            canvas,
            gl,
            a_positions:a_positions_loc,
            u_matrix:u_matrix_loc,
            a_color:a_color_loc, 
            camera: ui_control,
            spider: Spider::new(),
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

        // starts animations - put it all inside the same function?
        self.spider.animate_front_legs(0.026); // all transformations here               
        //self.spider.animate_back_legs(0.026);
        //self.spider.animate_middle_legs(0.026);

        let mut upper_leg_model_matrix = m4::projection(self.canvas.client_width() as f32, self.canvas.client_height() as f32, 600.);
        
        // translations for scene graph
        upper_leg_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
            self.spider.get_curr_pivot_point_of_upper_leg().0, 
            self.spider.get_curr_pivot_point_of_upper_leg().1, 
            self.spider.get_curr_pivot_point_of_upper_leg().2
        ));

        let mut middle_leg_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
            - self.spider.pivot_point.0 * 2.,
            0.,
            0.
        ));
       
        upper_leg_model_matrix = m4::z_rotate_3_d( // is that better to put it inside animate() ?
            upper_leg_model_matrix,
            m4::z_rotation(deg_to_rad(self.spider.z_acc_rotation).into())
        );
       
        upper_leg_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
            - self.spider.pivot_point.0,
            - self.spider.pivot_point.1,
            - self.spider.pivot_point.2
        ));

        middle_leg_model_matrix = m4::z_rotate_3_d( // is that better to put it inside animate() ?
            middle_leg_model_matrix,
            m4::z_rotation(deg_to_rad(self.spider.z_acc_rotation * -1.).into())
        );

        middle_leg_model_matrix = m4::translate_3_d(middle_leg_model_matrix, m4::translation(
            0.,
            - self.spider.pivot_point.1,
            0.
        ));

        let mut bottom_leg_model_matrix = m4::translate_3_d(middle_leg_model_matrix, m4::translation(
            - self.spider.pivot_point.0,
            - self.spider.pivot_point.1,
            0.
        ));
              
        bottom_leg_model_matrix = m4::z_rotate_3_d(bottom_leg_model_matrix, m4::z_rotation(deg_to_rad(180.).into()));
        
        bottom_leg_model_matrix = m4::translate_3_d(bottom_leg_model_matrix, m4::translation(
            - self.spider.pivot_point.0,
            - self.spider.pivot_point.1,
            - self.spider.pivot_point.2
        ));
        
        bottom_leg_model_matrix = m4::z_rotate_3_d(bottom_leg_model_matrix, m4::z_rotation(deg_to_rad(self.spider.z_acc_rotation * -1.).into()));
        
        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &upper_leg_model_matrix);

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


        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &middle_leg_model_matrix);

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
            
        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &bottom_leg_model_matrix);

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
}






