use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlRenderingContext as Gl};

use crate::{
    data_structures::{
        get_colors,  
        get_base_leg_colors, 
        get_body_colors, 
        get_body_data, get_head_data
    }, 
    constants::*, setup_ui_control::SpiderControl, leg::Leg, gpu_interface::GpuInterface, matrix_stack::MatrixStack, log
};


#[derive(PartialEq)]
pub enum LegType {
    Frontal,
    Back,
    Middle
}

pub struct Spider {
    pub colors: [u8; 108],
    pub base_leg_colors: [u8; 54],
    pub body_colors: [u8; 270],
    pub body_data: [f32; 270],
    pub head_data: [f32; 270], // use body_colors
    pub speed: f32,
    pub body_x_acc_rotation: f32,
    pub body_y_acc_rotation: f32,
    pub body_z_acc_rotation: f32, 
    pub frontal_legs: [Leg; 2], 
    pub back_legs: [Leg; 2],
    pub middle_legs: [Leg; 4],
    control: SpiderControl,
    pub animation_matrix_stack: MatrixStack
}

impl Spider {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        //ITERATE OVER RANGES OR NUMBER OF LEGS FOR LESS CODE or LEG BUILDER
        let mut frontal_legs = [
            Leg::new(
                LegType::Frontal, 
                ( 
                    BODY_WIDTH - FRONTAL_LEG_INSET, 
                    BODY_HEIGHT / 2.15,
                    BODY_FRONTAL_DEPTH_OFFSET / 2.                
                ),
                0
            ),

            Leg::new(
                LegType::Frontal, 
                ( 
                    BODY_WIDTH - FRONTAL_LEG_INSET, 
                    BODY_HEIGHT / 2.15,
                    BODY_DEPTH - BODY_FRONTAL_DEPTH_OFFSET / 2.
                ),
                1   
            )
        ];

        let mut back_legs = [
            Leg::new(
                LegType::Back, 
                ( 
                    0., 
                    BODY_HEIGHT / 2.75,
                    BODY_DEPTH - BODY_BACK_DEPTH_OFFSET / 2.
                ),
                0
            ),

            Leg::new(
                LegType::Back, 
                ( 
                    0.,
                    BODY_HEIGHT / 2.75,
                    BODY_BACK_DEPTH_OFFSET / 2.
                ),
                1,
            )
        ];

        let mut middle_legs = [
            Leg::new(
                LegType::Middle, 
                (  
                    BODY_WIDTH / 2. / 2.,
                    BODY_HEIGHT / 2.1,
                    0.
                ),
                0
            ),
           
            Leg::new(
                LegType::Middle, 
                (  
                    BODY_WIDTH - (BODY_WIDTH / 2. / 2.),
                    BODY_HEIGHT / 2.1,
                    0.
                ),
                0
            ),

            // OTHER SIDE
            Leg::new(
                LegType::Middle, 
                (  
                    BODY_WIDTH / 2. / 2.,
                    BODY_HEIGHT / 2.1,
                    BODY_DEPTH
                ),
                0
            ),
            
            Leg::new(
                LegType::Middle, 
                (  
                    BODY_WIDTH - (BODY_WIDTH / 2. / 2.),
                    BODY_HEIGHT / 2.1,
                    BODY_DEPTH
                ),
                0
            ),
        ];
        
        // here i can active specific legs to start animation
        frontal_legs[0].start_moving(); 
        frontal_legs[1].start_moving();
        back_legs[0].start_moving(); 
        back_legs[1].start_moving();
        middle_legs[0].start_moving(); 
        //middle_legs[1].start_moving(); 
        // middle_legs[2].start_moving(); 
        // middle_legs[3].start_moving(); 

        Self { 
            control: SpiderControl::new(&canvas), // CharControl
            frontal_legs,
            middle_legs,
            back_legs,       
            speed: 10., 
            body_z_acc_rotation: 0.,
            body_x_acc_rotation: 0.,
            body_y_acc_rotation: 0.,
            body_data: get_body_data(), // call it directly on the code
            body_colors: get_body_colors(), // call it directly on the code
            head_data: get_head_data(),
            colors: get_colors(), // call it directly on the code
            base_leg_colors: get_base_leg_colors(),
            animation_matrix_stack: MatrixStack { stack: Vec::new() },
        }
    }

    pub fn animate_body(&self, gpu_interface: &GpuInterface, body_model_matrix: &[f32; 16], positions_buffer: &WebGlBuffer, colors_buffer: &WebGlBuffer) {
        gpu_interface.send_positions_to_gpu(&self.body_data, &positions_buffer);
        gpu_interface.send_colors_to_gpu(&self.body_colors, &colors_buffer);
        
        gpu_interface.consume_data(
            self.body_data.len() as i32 / 3, 
            Gl::TRIANGLES, 
            &body_model_matrix
        );
    }

    pub fn animate_front_legs(&mut self, gpu_interface: &GpuInterface, body_model_matrix: &[f32; 16], positions_buffer: &WebGlBuffer, colors_buffer: &WebGlBuffer) {
        for (i, leg) in self.frontal_legs.iter_mut().enumerate() {   
            
            let animation_model_matrices = leg.walk_animate(
                &body_model_matrix, // reference
                &self.control.direction.try_borrow().unwrap(),
                i,
                &mut self.animation_matrix_stack
            );

            for (j, model_matrix) in animation_model_matrices.iter().enumerate() {                
                gpu_interface.send_positions_to_gpu(&leg.vertex_data[j], positions_buffer);
                
                if i == 2 { // only for base leg 
                    gpu_interface.send_colors_to_gpu(&self.base_leg_colors, colors_buffer);
                } else {
                    gpu_interface.send_colors_to_gpu(&self.colors, colors_buffer);
                }
                
                gpu_interface.consume_data(
                    leg.vertex_data[j].len() as i32 / 3, 
                    Gl::TRIANGLES, 
                    &model_matrix.unwrap()
                );
            }
            
        }
    }

    pub fn animate_back_legs(&mut self, gpu_interface: &GpuInterface, body_model_matrix: &[f32; 16], positions_buffer: &WebGlBuffer, colors_buffer: &WebGlBuffer) {
        for (i, leg) in self.back_legs.iter_mut().enumerate() {   
            
            let animation_model_matrix = leg.walk_animate(
                &body_model_matrix,
                &self.control.direction.try_borrow().unwrap(), 
                i,
                &mut self.animation_matrix_stack
            );
            
            for (j, model_matrix) in animation_model_matrix.iter().enumerate() {                
                gpu_interface.send_positions_to_gpu(&leg.vertex_data[j], positions_buffer);
                
                if i == 2 { // only for base leg 
                    gpu_interface.send_colors_to_gpu(&self.base_leg_colors, colors_buffer);
                } else {
                    gpu_interface.send_colors_to_gpu(&self.colors, colors_buffer);
                }
                
                gpu_interface.consume_data(
                    leg.vertex_data[j].len() as i32 / 3, 
                    Gl::TRIANGLES, 
                    &model_matrix.unwrap()
                );
            }            
        }
    }

    pub fn animate_middle_legs(&mut self, gpu_interface: &GpuInterface, body_model_matrix: &[f32; 16], positions_buffer: &WebGlBuffer, colors_buffer: &WebGlBuffer) {
        for (i, leg) in self.middle_legs.iter_mut().enumerate() {   
            
            let animation_model_matrix = leg.walk_animate(
                &body_model_matrix,
                &self.control.direction.try_borrow().unwrap(),
                i,
                &mut self.animation_matrix_stack
            );
            
            for (j, model_matrix) in animation_model_matrix.iter().enumerate() {                
                gpu_interface.send_positions_to_gpu(&leg.vertex_data[j], positions_buffer);
                
                if i == 2 { // only for base leg 
                    gpu_interface.send_colors_to_gpu(&self.base_leg_colors, colors_buffer);
                } else {
                    gpu_interface.send_colors_to_gpu(&self.colors, colors_buffer);
                }
                
                gpu_interface.consume_data(
                    leg.vertex_data[j].len() as i32 / 3, 
                    Gl::TRIANGLES, 
                    &model_matrix.unwrap()
                );
            }            
        }
    } 
}


