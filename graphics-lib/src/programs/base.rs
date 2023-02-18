use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::{shaders, log};

pub enum Program {
    Program1,
    Program2,
    Program3,
}
pub struct ProgramBuilder {}

impl ProgramBuilder {

    pub fn build(context: &WebGlRenderingContext, program_indx: Program) -> WebGlProgram {

        let vertex_source: &str;
        let frag_source: &str;

        match program_indx {
            Program::Program1 => {
                vertex_source = shaders::vertex::base::SHADER;
                frag_source = shaders::fragment::base::SHADER;
            },
            Program::Program2 => {
                vertex_source = shaders::vertex::base_2::SHADER;
                frag_source = shaders::fragment::base_2::SHADER;
            },
            Program::Program3 => {
                vertex_source = shaders::vertex::base::SHADER;
                frag_source = shaders::fragment::base::SHADER;
            },
        }

        let vert_shader = ProgramBuilder::compile_shader(
            context, 
            WebGlRenderingContext::VERTEX_SHADER,
            vertex_source
        ).unwrap();
    
        let frag_shader = ProgramBuilder::compile_shader(
            context, 
            WebGlRenderingContext::FRAGMENT_SHADER, 
            frag_source
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



