use std::panic;
//use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

use console_error_panic_hook;
use js_sys::WebAssembly;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGlProgram, WebGlRenderingContext, HtmlCanvasElement};
use web_sys::WebGl2RenderingContext as Gl;
//use quad_rand as qrand;

mod shaders;
mod setup;
mod programs;
mod webgl_utils;

use setup::initialize_webgl_context;
use webgl_utils::resize_canvas_to_display_size;
use programs::base::ProgramBuilder;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);

}


#[wasm_bindgen]
pub struct GraphicsClient {
    context: WebGlRenderingContext,
    programs: Vec<WebGlProgram>,
    storage_positions: Vec<f32>,
    memory_buffer: JsValue,
    canvas_elem: HtmlCanvasElement
}

unsafe impl Send for GraphicsClient {}

#[wasm_bindgen]
impl GraphicsClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let (context, canvas) = initialize_webgl_context().unwrap(); // deep into problems (Err)

        resize_canvas_to_display_size(&context, &canvas);

        let mut programs = Vec::new();
        let program = ProgramBuilder::build(&context); // create and compile shaders & link the program with these shaders
        programs.push(program);

        let memory_buffer = wasm_bindgen::memory() 
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    
        let storage_positions = GraphicsClient::get_centered_sized_quarter_or_half_canvas(
            &canvas, 
            Some(50.), 
            Some(50.)
        ); // TRUNC VALUE
         
        Self { 
            context, 
            programs,
            storage_positions,
            memory_buffer,
            canvas_elem: canvas
        }
    }


    // think of cronology of functions
    pub fn render(&mut self) {
        let curr_program = &self.programs[0]; // error handling (?)
        self.context.use_program(Some(curr_program));

        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        
        let pos_buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&pos_buffer));

        let should_redraw = resize_canvas_to_display_size(&self.context, &self.canvas_elem);

        if should_redraw {
            let new_geometry_pos = GraphicsClient::get_centered_sized_quarter_or_half_canvas(
                &self.canvas_elem, 
                None, 
                None
            ); // different sizes

            self.storage_positions = new_geometry_pos;    
        }
        
        let resolution_attr_loc = self.context.get_uniform_location(&curr_program, "uResolution").unwrap();
        self.context.uniform2f(Some(&resolution_attr_loc), self.canvas_elem.client_width() as f32 as f32, self.canvas_elem.client_height() as f32);
       
        
        // bind data to buffer as float32Array ////////////////////////////////////////////////////////////////////////////////
        
        let pos_memory_loc = self.storage_positions.as_ptr() as u32 / 4;
        let pos_ptr = js_sys::Float32Array::new(&self.memory_buffer)
            .subarray(pos_memory_loc, pos_memory_loc + self.storage_positions.len() as u32);

        ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

        let primitive = Gl::TRIANGLES;
        
        let position_attr_loc = self.context.get_attrib_location(&curr_program, "aPosition");
        self.context.vertex_attrib_pointer_with_i32(
            position_attr_loc as u32, 
            2, 
            Gl::FLOAT, 
            false, 
            0, 
            0,
            // ARRAY_BUFFER = last binded buffer
        ); // use the same buffer to draw more stuff [STRIDE] 
        
        self.context.enable_vertex_attrib_array(position_attr_loc as u32);
        
        self.context.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &pos_ptr, Gl::STATIC_DRAW); // it could a different primitive

        self.draw(self.storage_positions.len() as i32, primitive);

    }


    fn draw(&self, vert_count: i32, mode: u32) {
        self.context.draw_arrays(mode, 0, vert_count);
    }

    fn get_centered_sized_quarter_or_half_canvas(canvas: &HtmlCanvasElement, geom_width: Option<f32>, geom_height: Option<f32>) -> Vec<f32> {
        let center_x = (canvas.client_width() / 2) as f32;
        let center_y = (canvas.client_height() / 2) as f32;
        

        let (x, x2) = match geom_width {
            Some(size) => {
                
                let x = center_x - (size / 2.);
                let x2 = x + size;   
                
                (x, x2)
            },
            None => {
                
                let size = center_x;
                
                let x = center_x - (size / 2.);
                let x2 = x + size;    

                (x, x2)
            },
        };

        let (y, y2) = match geom_height {
            Some(size) => {
                let y = center_y + (size / 2.);
                let y2 = y - size;

                (y, y2)
            },
            None => {
                let size= center_y;

                let y = center_y + (size / 2.);
                let y2 = y - size;

                (y, y2)
            },
        };

        // let x = center_x - (geom_width / 2.);
        // let x2 = x + width;
        
        // let y = center_y + (geom_height / 2.);
        // let y2 = y - height;

        vec![
            x, y,
            x, y2,
            x2, y,
            x2, y,
            x, y2,
            x2, y2
        ]
    }

    // fn build_vec_geom(geometry: Geometry, width: f32, height: f32, canvas: &HtmlCanvasElement) -> Vec<f32> { // in pxls
     
    //     match geometry {
    //         Geometry::TRIANGLE => {
    //             let x = (quad_rand::gen_range(0., 1.01) * 2. -1.) as f32;
    //             let x2 = x + width;
    //             let y = (quad_rand::gen_range(0., 1.01) * 2. -1.) as f32;
    //             let y2 = y + height;

    //             vec![
    //                 x, y2,
    //                 x, y,
    //                 x2, y2
    //             ]
    //         },
    //         Geometry::QUARTER => {
    //             let center_x = (canvas.client_width() / 2) as f32;
    //             let center_y = (canvas.client_height() / 2) as f32;
              
    //             let x = center_x - (width / 2.);
    //             let x2 = x + width;
             
    //             let y = center_y + (height / 2.);
    //             let y2 = y - height;

    //             vec![
    //                 x, y,
    //                 x, y2,
    //                 x2, y,
    //                 x2, y,
    //                 x, y2,
    //                 x2, y2
    //             ]
    //         },
    //         Geometry::LINE => {
    //             vec![
    //                 quad_rand::gen_range(0., 1.01) * 2. -1., quad_rand::gen_range(0., 1.01) * 2. -1.,
    //                 quad_rand::gen_range(0., 1.01) * 2. -1., quad_rand::gen_range(0., 1.01) * 2. -1.
    //             ]
    //         },
    //     }
    // }
}

enum Geometry {
    TRIANGLE,
    QUARTER,
    LINE
}





