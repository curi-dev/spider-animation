use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{WebGlUniformLocation, WebGlProgram, WebGlRenderingContext, WebGlBuffer};
use web_sys::WebGlRenderingContext as Gl;

//use crate::log;

pub struct GpuInterface {
    a_positions: i32,
    u_matrix: WebGlUniformLocation,
    u_normal_mat: WebGlUniformLocation,
    u_color: WebGlUniformLocation,
    u_reverse_light: WebGlUniformLocation,
    a_normal: i32,
    pub gl: WebGlRenderingContext
}

impl GpuInterface {
    pub fn new(gl: WebGlRenderingContext, program: &WebGlProgram) -> Self {
        let a_positions_loc = gl.get_attrib_location(&program, "aPosition");
        let u_matrix_loc = gl.get_uniform_location(&program, "uMatrix").unwrap();
        let u_color_loc = gl.get_uniform_location(&program, "uColor").unwrap();
        let u_reverse_light_loc = gl.get_uniform_location(&program, "uReverseLight").unwrap();
        let u_normal_mat_loc = gl.get_uniform_location(&program, "uNormalMatrix").unwrap();
        let a_normal_loc = gl.get_attrib_location(&program, "aNormal");


        Self {
            gl, 
            a_positions: a_positions_loc, 
            u_matrix: u_matrix_loc, 
            a_normal: a_normal_loc,
            u_color: u_color_loc,
            u_reverse_light: u_reverse_light_loc,
            u_normal_mat: u_normal_mat_loc,
        }
    }

    pub fn send_positions_to_gpu(&self, positions: &[f32], positions_buffer: &WebGlBuffer) {
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
        
        self.gl.enable_vertex_attrib_array(self.a_positions as u32);
    }
    
    pub fn send_normals_to_gpu(&self, normals: &[f32], normals_buffer: &WebGlBuffer) {
        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(normals_buffer));
        
        // i tell GPU how to get the values from a specific attribute 
        // the attribute is basicly the variable used inside the shaders
        self.gl.vertex_attrib_pointer_with_i32(
            self.a_normal as u32, 
            3, 
            Gl::FLOAT, // is that right? 
            false, 
            0, 
            0,
        );

        // persist this memory buffer?
        let memory_buffer_view = wasm_bindgen::memory() 
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

        let ptr_mem_loc = normals.as_ptr() as u32 / 4;
        let coords_ptr = js_sys::Float32Array::new(&memory_buffer_view)
            .subarray(ptr_mem_loc, ptr_mem_loc + normals.len() as u32);

        self.gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &coords_ptr, Gl::STATIC_DRAW);
        
        self.gl.enable_vertex_attrib_array(self.a_normal as u32); 
    }

    pub fn consume_data(
        &self, vert_count: i32, 
        mode: u32, 
        model_matrix: &[f32; 16],
        normal_matrix: &[f32; 16],
        light: &[f32; 3],
        color: (f32, f32, f32)
    ) {
        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_matrix), false, model_matrix);
        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_normal_mat), false, normal_matrix); 
        //self.gl.uniform4f(Some(&self.u_color), 0.05, 0.05, 0.05, 1.);
        self.gl.uniform4f(Some(&self.u_color), color.0, color.1, color.2, 1.);

        let normalized_light = nalgebra::Vector3::new(light[0], light[1], light[2]).normalize();
        self.gl.uniform3fv_with_f32_array(Some(&self.u_reverse_light), &normalized_light.as_slice()); // understand this concept of reverse light
        
        self.gl.draw_arrays(mode, 0, vert_count);
    }

    // pub fn send_colors_to_gpu(&self, colors: &[u8], colors_buffer: &WebGlBuffer) {
    //     self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(colors_buffer));
        
    //     self.gl.vertex_attrib_pointer_with_i32(
    //         self.a_color as u32, 
    //         3, 
    //         Gl::UNSIGNED_BYTE, 
    //         true, 
    //         0, 
    //         0,
    //     );

    //     let memory_buffer_view = wasm_bindgen::memory() // persist this memory buffer?
    //     .dyn_into::<WebAssembly::Memory>()
    //     .unwrap()
    //     .buffer();

    //     let ptr_mem_loc = colors.as_ptr() as u32; // 4 bytes each
    //     let coords_ptr = js_sys::Uint8Array::new(&memory_buffer_view)
    //         .subarray(ptr_mem_loc, ptr_mem_loc + colors.len() as u32);
    //     self.gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &coords_ptr, Gl::STATIC_DRAW);
    //     self.gl.enable_vertex_attrib_array(self.a_positions as u32);
    // }
    
}