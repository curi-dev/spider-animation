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

        //self.send_positions_to_gpu(&self.spider.upper_and_middle_legs_data);
        self.send_positions_to_gpu(&self.spider.frontal_legs[0].upper_leg_data);

        // starts animations - put it all inside the same function?
        self.spider.animate_front_legs(0.026); // all transformations here               
        //self.spider.animate_back_legs(0.026);
        //self.spider.animate_middle_legs(0.026);

        //let mut upper_leg_model_matrix = m4::projection(self.canvas.client_width() as f32, self.canvas.client_height() as f32, 600.);
        
        let aspect = (self.canvas.client_width() / self.canvas.client_height()) as f32;
        
        let mut upper_leg_model_matrix = m4::perspective(
            deg_to_rad(DEFAULT_FIELD_OF_VIEW_IN_RADIANS),
            aspect,
            DEFAULT_Z_NEAR,
            DEFAULT_Z_FAR
        );
        // initial displacement
        upper_leg_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
            INITIAL_LEG_DISPLACEMENT_X, 
            INITIAL_LEG_DISPLACEMENT_Y,
            INITIAL_LEG_DISPLACEMENT_Z, 
        ));

        // ui control transformations -> create function inside graphics client using ui control?
        let ui_rotation_x = self.ui_control.acc_x_rotation.try_borrow().unwrap();
        let ui_rotation_y = self.ui_control.acc_y_rotation.try_borrow().unwrap();
        let ui_translate_z = self.ui_control.acc_z_translation.try_borrow().unwrap();

        upper_leg_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
            0., 
            0., 
            *ui_translate_z 
        ));

        upper_leg_model_matrix = m4::x_rotate_3_d( // is that better to put it inside animate() ?
            upper_leg_model_matrix,
            m4::x_rotation(deg_to_rad(*ui_rotation_x).into())
        );

        upper_leg_model_matrix = m4::y_rotate_3_d( // is that better to put it inside animate() ?
            upper_leg_model_matrix,
            m4::y_rotation(deg_to_rad(*ui_rotation_y).into())
        );
        ///////////////////////////////////////////////////////////////////////////////////////////
        
        // translations for scene graph
        // upper_leg_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
        //     FRONTAL_UPPER_LEG_WIDTH, // advance width
        //     FRONTAL_UPPER_LEG_SMALL_HEIGHT / 2., // adjust height
        //     LEG_DEPTH / 2. // adjust depth -> now its on pivot point
        // ));

        let mut middle_leg_model_matrix = m4::translate_3_d(
            upper_leg_model_matrix, 
            m4::translation(
            FRONTAL_MIDDLE_LEG_WIDTH * -1., // adjust distance by width 
            0., // same y
            (LEG_DEPTH / 2.) * -1. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
            )
        );
       
        // upper_leg_model_matrix = m4::z_rotate_3_d( // is that better to put it inside animate() ?
        //     upper_leg_model_matrix,
        //     m4::z_rotation(deg_to_rad(self.spider.z_acc_rotation).into())
        // );
       
        // upper_leg_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
        //     FRONTAL_UPPER_LEG_WIDTH * -1., 
        //     (FRONTAL_UPPER_LEG_SMALL_HEIGHT / 2.) * -1., 
        //     (LEG_DEPTH / 2.) * -1.
        // ));

        // middle_leg_model_matrix = m4::z_rotate_3_d( // is that better to put it inside animate() ?
        //     middle_leg_model_matrix,
        //     m4::z_rotation(deg_to_rad(self.spider.z_acc_rotation * -1.).into())
        // );

        // middle_leg_model_matrix = m4::translate_3_d(middle_leg_model_matrix, m4::translation(
        //     FRONTAL_MIDDLE_LEG_WIDTH * -1.,
        //     (FRONTAL_MIDDLE_LEG_SMALL_HEIGHT / 2.) * -1.,
        //     0.
        // ));

        let mut bottom_leg_model_matrix = m4::translate_3_d(
            middle_leg_model_matrix, 
            m4::translation(
            FRONTAL_BOTTOM_LEG_WIDTH * -1.,
            FRONTAL_MIDDLE_LEG_BIG_HEIGHT / 2.,
            0.,
            )
        );

        
        //bottom_leg_model_matrix = m4::z_rotate_3_d(bottom_leg_model_matrix, m4::z_rotation(deg_to_rad(self.spider.z_acc_rotation * -1.).into()));    
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
        
        self.consume_data(self.spider.frontal_legs[0].upper_leg_data.len() as i32 / 3, Gl::TRIANGLES);


        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &middle_leg_model_matrix);

        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&positions_buffer));

        self.gl.vertex_attrib_pointer_with_i32(
            self.a_positions as u32, 
            3, 
            Gl::FLOAT, 
            false, 
            0, 
            0,
        );

        //self.send_positions_to_gpu(&self.spider.upper_and_middle_legs_data);
        self.send_positions_to_gpu(&self.spider.frontal_legs[0].middle_leg_data);

        self.consume_data(self.spider.frontal_legs[0].upper_leg_data.len() as i32 / 3, Gl::TRIANGLES);
        
        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&positions_buffer));

        self.gl.vertex_attrib_pointer_with_i32(
            self.a_positions as u32, 
            3, 
            Gl::FLOAT, 
            false, 
            0, 
            0,
        );

        //self.send_positions_to_gpu(&self.spider.bottom_legs_data);
        self.send_positions_to_gpu(&self.spider.frontal_legs[0].bottom_leg_data);
            
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
        self.send_colors_to_gpu(&self.spider.base_leg_colors);

        self.consume_data(self.spider.frontal_legs[0].bottom_leg_data.len() as i32 / 3, Gl::TRIANGLES);

        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&positions_buffer));

        self.gl.vertex_attrib_pointer_with_i32(
            self.a_positions as u32, 
            3, 
            Gl::FLOAT, 
            false, 
            0, 
            0,
        );

        self.send_positions_to_gpu(&self.spider.body_data);

        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&colors_buffer));
        
        self.gl.vertex_attrib_pointer_with_i32(
            self.a_color as u32, 
            3, 
            Gl::UNSIGNED_BYTE, 
            true, 
            0, 
            0,
        );
        self.send_colors_to_gpu(&self.spider.body_colors);

        let mut body_model_matrix = m4::perspective(
            deg_to_rad(DEFAULT_FIELD_OF_VIEW_IN_RADIANS),
            aspect,
            DEFAULT_Z_NEAR,
            DEFAULT_Z_FAR
        );

        body_model_matrix = m4::translate_3_d(upper_leg_model_matrix, m4::translation(
            INITIAL_BODY_DISPLACEMENT_X, 
            INITIAL_BODY_DISPLACEMENT_Y,
            INITIAL_BODY_DISPLACEMENT_Z, 
        ));

        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, &body_model_matrix);

        self.consume_data(self.spider.body_data.len() as i32 / 3, Gl::TRIANGLES);
       
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






