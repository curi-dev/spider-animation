use std::panic;
use console_error_panic_hook;
use js_sys::WebAssembly;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGlRenderingContext, HtmlCanvasElement, WebGlUniformLocation, HtmlImageElement, WebGlBuffer};
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

use setup::initialize_webgl_context;
use modules::m4::m4::M4 as m4;
use webgl_utils::resize_canvas_to_display_size;
use programs::base::ProgramBuilder;
use setup_ui_control::SetupUiControl;

use crate::constants::*;
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
    ui_control: SetupUiControl,
    spider: spider::Spider,
}

unsafe impl Send for GraphicsClient {}

#[wasm_bindgen]
impl GraphicsClient {

    #[wasm_bindgen(constructor)]
    pub fn new(_image: HtmlImageElement) -> Self {    
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let (gl, canvas, ) = initialize_webgl_context().unwrap(); // transfer to initialize_events()
        
        // create and compile shaders & link the program with these shaders (can pass the shaders here [dependency inversion]) 
        let program = ProgramBuilder::build(&gl, Program::Program3); // refactor!

        let a_positions_loc = gl.get_attrib_location(&program, "aPosition");
        let u_matrix_loc = gl.get_uniform_location(&program, "uMatrix").unwrap();
        let a_color_loc = gl.get_attrib_location(&program, "aColor");

        gl.use_program(Some(&program));

        let ui_control = SetupUiControl::new(&canvas);

        Self {
            canvas,
            gl,
            ui_control,
            a_positions:a_positions_loc,
            u_matrix:u_matrix_loc,
            a_color:a_color_loc, 
            spider: Spider::new(),
        }      
    }


    pub fn render(&mut self) {          
        //deltatime = deltatime / 1000.;
        self.gl.clear(Gl::COLOR_BUFFER_BIT);
        resize_canvas_to_display_size(&self.gl, &self.canvas);
        let aspect = (self.canvas.client_width() / self.canvas.client_height()) as f32;

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
        let positions_buffer = self.gl.create_buffer().unwrap(); 
        let colors_buffer = self.gl.create_buffer().unwrap(); 
        
        self.send_positions_to_gpu(&self.spider.body_data, &positions_buffer);
        self.send_colors_to_gpu(&self.spider.body_colors, &colors_buffer);
        
        self.consume_data(
            self.spider.body_data.len() as i32 / 3, 
            Gl::TRIANGLES, 
            &body_model_matrix
        );
        // ROUND ONE DONE -> BODY

        self.send_positions_to_gpu(&self.spider.head_data, &positions_buffer);
        self.send_colors_to_gpu(&self.spider.body_colors, &colors_buffer);
        
        let head_model_matrix = m4::translate_3_d(body_model_matrix, m4::translation(
            BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 
            3., 
            BODY_DEPTH / 2. - HEAD_DEPTH / 2. 
        ));

        self.consume_data(
            self.spider.head_data.len() as i32 / 3, 
            Gl::TRIANGLES, 
            &head_model_matrix
        );

        for (i, leg) in self.spider.frontal_legs.iter().enumerate() {
            let positions_buffer = self.gl.create_buffer().unwrap(); 
            let colors_buffer = self.gl.create_buffer().unwrap(); 
            let pivot_point_model_matrix = leg.set_pivot_point(&body_model_matrix);
            let initial_transformations_model_matrix = leg.set_initial_transformations(&pivot_point_model_matrix, i);
            for (j, model_matrix) in initial_transformations_model_matrix.iter().enumerate() {                
                self.send_positions_to_gpu(&leg.vertex_data[j], &positions_buffer);

                if i == 2 { // only for base leg 
                    self.send_colors_to_gpu(&self.spider.base_leg_colors, &colors_buffer);
                } else {
                    self.send_colors_to_gpu(&self.spider.colors, &colors_buffer);
                }
                
                self.consume_data(
                    leg.vertex_data[j].len() as i32 / 3, 
                    Gl::TRIANGLES, 
                    model_matrix
                );
            }            
        }

        for (i, leg) in self.spider.back_legs.iter().enumerate() {
            let positions_buffer = self.gl.create_buffer().unwrap(); 
            let colors_buffer = self.gl.create_buffer().unwrap(); 
            let pivot_point_model_matrix = leg.set_pivot_point(&body_model_matrix);
            let initial_transformations_model_matrix = leg.set_initial_transformations(&pivot_point_model_matrix, i);
            for (j, model_matrix) in initial_transformations_model_matrix.iter().enumerate() {    
                self.send_positions_to_gpu(&leg.vertex_data[j], &positions_buffer);

                if i == 2 { // only for base leg 
                    self.send_colors_to_gpu(&self.spider.base_leg_colors, &colors_buffer);
                } else {
                    self.send_colors_to_gpu(&self.spider.colors, &colors_buffer);
                }
                
                self.consume_data(
                    leg.vertex_data[j].len() as i32 / 3, 
                    Gl::TRIANGLES, 
                    model_matrix
                );
            }            
        }

        for (i, leg) in self.spider.middle_legs.iter().enumerate() {
            let positions_buffer = self.gl.create_buffer().unwrap(); 
            let colors_buffer = self.gl.create_buffer().unwrap(); 
            let pivot_point_model_matrix = leg.set_pivot_point(&body_model_matrix);
            let initial_transformations_model_matrix = leg.set_initial_transformations(&pivot_point_model_matrix, i);
            for (j, model_matrix) in initial_transformations_model_matrix.iter().enumerate() {    
                self.send_positions_to_gpu(&leg.vertex_data[j], &positions_buffer);

                if i == 2 { // only for base leg 
                    self.send_colors_to_gpu(&self.spider.base_leg_colors, &colors_buffer);
                } else {
                    self.send_colors_to_gpu(&self.spider.colors, &colors_buffer);
                }
                
                self.consume_data(
                    leg.vertex_data[j].len() as i32 / 3, 
                    Gl::TRIANGLES, 
                    model_matrix
                );
            }            
        }
    }

    fn send_positions_to_gpu(&self, positions: &[f32], positions_buffer: &WebGlBuffer) {
        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(positions_buffer));

        self.gl.vertex_attrib_pointer_with_i32(
            self.a_positions as u32, 
            3, 
            Gl::FLOAT, 
            false, 
            0, 
            0,
        );

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

    fn send_colors_to_gpu(&self, colors: &[u8], colors_buffer: &WebGlBuffer) {
        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(colors_buffer));
        
        self.gl.vertex_attrib_pointer_with_i32(
            self.a_color as u32, 
            3, 
            Gl::UNSIGNED_BYTE, 
            true, 
            0, 
            0,
        );

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
    
    fn consume_data(&self, vert_count: i32, mode: u32, model_matrix: &[f32; 16]) {
        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, model_matrix);
        
        self.gl.draw_arrays(mode, 0, vert_count);
    }
}






