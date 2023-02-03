use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::shaders;


pub struct ProgramBuilder {}

impl ProgramBuilder {

    pub fn build(context: &WebGlRenderingContext) -> WebGlProgram {

        let vert_shader = ProgramBuilder::compile_shader(
            context, 
            WebGlRenderingContext::VERTEX_SHADER,
            shaders::vertex::base::SHADER
        ).unwrap();
    
        let frag_shader = ProgramBuilder::compile_shader(
            context, 
            WebGlRenderingContext::FRAGMENT_SHADER, 
            shaders::fragment::base::SHADER
        ).unwrap(); // ?
    
        // link the program and shaders
        let program = ProgramBuilder::link_program(context, &vert_shader, &frag_shader).unwrap(); 
        
        program
    }

    fn compile_shader(
        context: &WebGlRenderingContext,
        shader_type: u32,
        source: &str
    ) -> Result<WebGlShader, String> {
    
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
               
        context.shader_source(&shader, source);
        context.compile_shader(&shader);
        
        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader"))
            )
        }
    }
    
    fn link_program(
        context: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        
        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);
    
        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object"))
            )
        }
    }
}



