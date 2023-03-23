use std::ops::Range;
use nalgebra::{SVD, Matrix4};

use crate::{
    spider::LegType, 
    data_structures::*, 
    m4::{M4 as m4}, 
    constants::*, 
    webgl_utils::deg_to_rad, 
    setup_ui_control::Move, 
    log
};


pub struct Leg {
    pub vertex_data: Vec<Vec<f32>>,
    pub body_clamp_point: (f32, f32, f32),
    pub position: LegType,
    pub upper_last_pos_model_mat: Option<[f32; 16]>,
    
    pub is_moving: bool,
    move_cycle: u8,
    upper_side_y_move_cycle: u8,
    upper_side_z_move_cycle: u8,
    
    pub upper_side_z_move_range: Range<f32>,
    pub upper_side_y_move_range: Range<f32>,
    
    pub upper_forward_move_range: Range<f32>,
    pub joint_forward_move_range: Range<f32>,
    pub base_forward_move_range: Range<f32>,

    pub upper_x_acc_rotation: f32,
    pub upper_y_acc_rotation: f32,
    pub upper_z_acc_rotation: f32,
    
    pub joint_x_acc_rotation: f32,
    pub joint_y_acc_rotation: f32,
    pub joint_z_acc_rotation: f32,
    
    pub base_x_acc_rotation: f32,
    pub base_y_acc_rotation: f32,
    pub base_z_acc_rotation: f32,
}

impl Leg {
    pub fn new(leg_position: LegType, body_clamp_point: (f32, f32, f32), move_cycle: u8) -> Self {
        
        // upper, joint and base
        let vertex_data = get_leg_data(&leg_position);

        let upper_forward_move_range: Range<f32>;
        if leg_position == LegType::Frontal {
            upper_forward_move_range = Range { start: -25., end: 5. };
        } else if leg_position == LegType::Middle {
            upper_forward_move_range = Range { start: -10., end: 5. };
        } else {
            upper_forward_move_range = Range { start: -25., end: 5. };
        }
    
        Self {
            vertex_data,
            body_clamp_point,
            position: leg_position,
            upper_last_pos_model_mat: None,
            
            is_moving: false,
            move_cycle, // 0 or 1
            upper_side_y_move_cycle: 0, // 0 or 1
            upper_side_z_move_cycle: 0, // 0 or 1
            
            upper_side_z_move_range: Range { start: 0., end: 7. }, // alter range after half cycle complete
            upper_side_y_move_range: Range { start: -14., end: 14. },
            
            upper_x_acc_rotation: 0.,
            upper_y_acc_rotation: 0., 
            upper_z_acc_rotation: 0.,
          
            upper_forward_move_range,
            joint_forward_move_range: Range { start: 0., end: 17. },
            base_forward_move_range: Range { start: 0., end: 11. },
                       
            joint_x_acc_rotation: 0.,
            joint_z_acc_rotation: 0.,
            joint_y_acc_rotation: 0.,
            
            base_x_acc_rotation: 0.,
            base_z_acc_rotation: 0.,
            base_y_acc_rotation: 0.,
        }
    }

    pub fn stop_moving(&mut self) {
        if self.is_moving {
            self.is_moving = false;
        }
    }

    pub fn start_moving(&mut self) {
        if !self.is_moving {
            self.is_moving = true;
        }
    }

    pub fn switch_upper_side_y_move_cycle(&mut self) {
        if self.upper_side_y_move_cycle == 0 {
            self.upper_side_y_move_cycle = 1;
        } else {
            self.upper_side_y_move_cycle = 0;
        }
    }

    pub fn switch_upper_side_z_move_cycle(&mut self) {
        if self.upper_side_z_move_cycle == 0 {
            self.upper_side_z_move_cycle = 1;
        } else {
            self.upper_side_z_move_cycle = 0;
        }
    }

    pub fn switch_move_cycle(&mut self) {
        if self.move_cycle == 0 {
            self.move_cycle = 1;
        } else {
            self.move_cycle = 0;
        }
    }

    pub fn walk_animate(
        &mut self, 
        pre_matrix: &[f32; 16], 
        direction: &Move, 
        leg_i: usize,
    ) -> ([Option<[f32; 16]>; 3], [Option<[f32; 16]>; 3]) {


        let mut animation_models: [Option<[f32; 16]>; 3] = [None; 3];
        let mut accumulated_rotations_for_normal_transformations: [Option<[f32; 16]>; 3] = [None; 3];
        let mut animation_matrix_stack = Vec::new();

        // here i will extract only the rotation from the pre_matrix 
        let converted_pre_matrix = nalgebra::Matrix4::from_column_slice(pre_matrix);
        let svd = SVD::new(converted_pre_matrix, true, true);
       
        let r = svd.u.unwrap() * svd.v_t.unwrap().transpose();

        //let pre_matrix_rotation_matrix = r.try_normalize(std::f32::EPSILON).unwrap_or_default();
        //let pre_matrix_rotation_matrix_slice: [f32; 16] = pre_matrix_rotation_matrix.as_slice().try_into().unwrap();
        
        if let LegType::Frontal = self.position {
            println!("is frontal leg");

            for leg_part in 0..3 { // make it more semanthic
                
                if leg_part == 0 { // upper part

                    let clamping_model_matrix = m4::translate_3_d( 
                        *pre_matrix, // what happens with dereferencing a vector
                        m4::translation( 
                            self.body_clamp_point.0 - 0., 
                            self.body_clamp_point.1 - 0., 
                            self.body_clamp_point.2 - 0., 
                        )
                    );

                    let original_transformation = self.get_frontal_leg_transformation(
                        clamping_model_matrix, 
                        leg_part, 
                        leg_i 
                    );

                    if !self.is_moving {

                        animation_models[0] = Some(original_transformation);
                        animation_matrix_stack.push(original_transformation); 
                        accumulated_rotations_for_normal_transformations[0] = Some(m4::identity());

                    } else {
                        match direction {
                            Move::Forward => {
                                let displacement: f32;
                                if self.move_cycle == 0 {
                                    displacement = -1.;
                                } else {
                                    displacement = 1.;
                                }
                                
                                self.upper_z_acc_rotation += displacement; 
    
                                if !self.upper_forward_move_range.contains( &( self.upper_z_acc_rotation as f32 ) ) {
                                    self.switch_move_cycle();
                                }                     
                            },
                            Move::Left => {                                       
                                let rotation_y: f32;

                                if self.upper_side_y_move_cycle == 0 {
                                    rotation_y = 1.25;
                                } else {
                                    rotation_y = -1.25;
                                }
                                
                                self.upper_y_acc_rotation += rotation_y;  
     
                                if !self.upper_side_y_move_range.contains( &( self.upper_y_acc_rotation as f32 ) ) {
                                    self.switch_upper_side_y_move_cycle();                                 
                                }                        
                            },
                            Move::Right => {                                     
                                let rotation_y: f32;

                                if self.upper_side_y_move_cycle == 0 {
                                    rotation_y = -1.25;
                                } else {
                                    rotation_y = 1.25;
                                }
                                
                                self.upper_y_acc_rotation += rotation_y; // same values? 
     
                                if !self.upper_side_y_move_range.contains( &( self.upper_y_acc_rotation as f32 ) ) {
                                    self.switch_upper_side_y_move_cycle();                                
                                }                  
                            },
                            Move::Jump => {
                                println!("move jump!");
                            },
                            Move::Static => {
                                println!("no move!");
                            },
                            Move::SpinDown => {
                                println!("move spindown!");
                            },
                            Move::SpinUp => {
                                println!("move spinup");
                            },
                            Move::ZoomOut => {
                                println!("move zoomout")
                            },
                            Move::ZoomIn => {
                                println!("move zoomin")
                            },
                        }
    
                        ////////// animations below
                        let mut updated_model_matrix = m4::translate_3_d( // here is the pivot point
                            original_transformation,
                            m4::translation( 
                                FRONTAL_UPPER_LEG_WIDTH, 
                                FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.,
                                FRONTAL_UPPER_LEG_DEPTH / 2.
                            )
                        );

                        // the accumulate rotations are necessary to transform the normals
                        let accumulated_rotations = m4::multiply_mat(
                            m4::z_rotation( deg_to_rad( self.upper_z_acc_rotation * -1. ).into() ), 
                            m4::y_rotation( deg_to_rad( self.upper_y_acc_rotation ).into() )
                        );

                        let accumulated_rotations_from_decomposed = (
                            r * Matrix4::from_column_slice(&accumulated_rotations)
                        )
                        .try_normalize(std::f32::EPSILON)
                        .unwrap_or_default();
                        
                        accumulated_rotations_for_normal_transformations[0] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());
                        
                        updated_model_matrix = m4::multiply_mat(
                            updated_model_matrix, 
                            accumulated_rotations
                        );

                        updated_model_matrix = m4::translate_3_d( 
                            updated_model_matrix, 
                            m4::translation( 
                                - FRONTAL_UPPER_LEG_WIDTH, 
                                (FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.) * -1.,
                                (FRONTAL_UPPER_LEG_DEPTH / 2.) * -1.
                            )
                        );

                        ////////// animations above
                        
                        self.upper_last_pos_model_mat = Some(updated_model_matrix);
                        animation_models[0] = Some(updated_model_matrix); 
                        animation_matrix_stack.push(updated_model_matrix); 
                    }
                } else {
        
                    let clamping_model_matrix = animation_matrix_stack.last().unwrap();

                    let original_transformation = self.get_frontal_leg_transformation(
                        *clamping_model_matrix, 
                        leg_part, 
                        leg_i 
                    );

                    if !self.is_moving {
                        animation_models[leg_part] = Some(original_transformation);
                        animation_matrix_stack.push(original_transformation);
                        accumulated_rotations_for_normal_transformations[leg_part] = Some(m4::identity()); 

                        if leg_part == 2 {
                            animation_matrix_stack.drain(..);
                        }
                    } else {
                        if leg_part == 1 { // joint
                            match direction {
                                Move::Forward => {
                                    let displacement: f32;
                                    if self.move_cycle == 0 {
                                        displacement = -1.5;
                                    } else {
                                        displacement = 1.5;
                                    }
                                    
                                    self.joint_z_acc_rotation += displacement; 
                           
                                },
                                Move::Left => {
                                    let rotation_x: f32;
                                    let rotation_y: f32;
                                
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = 1.;
                                        rotation_y = -1.;
                                    
                                    } else {
                                        rotation_x = -1.;
                                        rotation_y = 1.;
                                     
                                    }
                                
                                    self.joint_x_acc_rotation += rotation_x; 
                                    self.joint_y_acc_rotation += rotation_y;
                                },
                                Move::Right => {
                                    let rotation_x: f32;
                                    let rotation_y: f32;
                                                                      
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = -1.;
                                        rotation_y = 1.;                                  
                                    } else {
                                        rotation_x = 1.;
                                        rotation_y = -1.;
                                    }
                                    
                                    self.joint_x_acc_rotation += rotation_x; 
                                    self.joint_y_acc_rotation += rotation_y; 
                                },
                                Move::Jump => {
                                    println!("move jump!");
                                },
                                Move::Static => {
                                    println!("no move!");
                                },
                                Move::SpinDown => {
                                    println!("move spindown!");
                                },
                                Move::SpinUp => {
                                    println!("move spinup");
                                },
                                Move::ZoomOut => {
                                    println!("move zoomout")
                                },
                                Move::ZoomIn => {
                                    println!("move zoomin")
                                },
                            }
                            
                            ////////// animations below
                            let mut updated_model_matrix = m4::translate_3_d( 
                                original_transformation, 
                                m4::translation( 
                                    FRONTAL_JOINT_LEG_WIDTH, 
                                    FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.,
                                    FRONTAL_JOINT_LEG_DEPTH / 2.
                                )
                            );

                            // only side animations below
                            let mut accumulated_rotations = m4::multiply_mat(
                                m4::x_rotation( deg_to_rad( self.joint_x_acc_rotation ).into() ), 
                                m4::y_rotation( deg_to_rad( self.joint_y_acc_rotation * -1. ).into() ) 
                            );

                            accumulated_rotations = m4::multiply_mat(
                                accumulated_rotations, 
                                m4::z_rotation( deg_to_rad( self.joint_z_acc_rotation ).into() ) 
                            );

                            let accumulated_rotations_from_decomposed = (
                                r * Matrix4::from_column_slice(&accumulated_rotations)
                            )
                            .try_normalize(std::f32::EPSILON)
                            .unwrap_or_default();
                            
                            accumulated_rotations_for_normal_transformations[1] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());
                            
                            updated_model_matrix = m4::multiply_mat(
                                updated_model_matrix, 
                                accumulated_rotations
                            );

                            updated_model_matrix = m4::translate_3_d( 
                                updated_model_matrix, 
                                m4::translation( 
                                    - FRONTAL_JOINT_LEG_WIDTH, 
                                    (FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) * -1.,
                                    (FRONTAL_JOINT_LEG_DEPTH / 2.) * -1.
                                )
                            );
                            ////////// animations above
                         
                            animation_models[1] = Some(updated_model_matrix);
                            animation_matrix_stack.push(updated_model_matrix); 
                        }

                        if leg_part == 2 { // base
                            match direction {
                                Move::Forward => {
                                    let displacement: f32;
                                    if self.move_cycle == 0 {
                                        if self.base_z_acc_rotation < 0. {
                                            displacement = 0.60;
                                        } else {
                                            displacement = 0.60;
                                        }
                                    } else {
                                        displacement = -0.60;
                                    }
                                    
                                    self.base_z_acc_rotation += displacement;  
                                         
                                },
                                Move::Left => {
                                    let rotation_x: f32;
                                    let rotation_y: f32;    
                                    
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = -1.;
                                        rotation_y = -1.;
                                    } else {
                                        rotation_x = 1.;
                                        rotation_y = 1.;
                                    }
                                    
                                    self.base_x_acc_rotation += rotation_x;
                                    self.base_y_acc_rotation += rotation_y;              
                                },
                                Move::Right => {
                                    let rotation_x: f32;    
                                    let rotation_y: f32;    
                                    
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = 1.;
                                        rotation_y = 1.;
                                    } else {
                                        rotation_x = -1.;
                                        rotation_y = -1.;
                                    }
                                    
                                    self.base_x_acc_rotation += rotation_x;  
                                    self.base_y_acc_rotation = rotation_y;
                                },
                                Move::Jump => {
                                    println!("move jump!");
                                },
                                Move::Static => {
                                    println!("no move!");
                                },
                                Move::SpinDown => {
                                    println!("move spindown!");
                                },
                                Move::SpinUp => {
                                    println!("move spinup");
                                },
                                Move::ZoomOut => {
                                    println!("move zoomout")
                                },
                                Move::ZoomIn => {
                                    println!("move zoomin")
                                },
                            }

                            ////////// animations below
                            
                            // only side animations below
                            let mut accumulated_rotations = m4::multiply_mat(
                                m4::x_rotation( deg_to_rad( self.base_x_acc_rotation ).into() ), 
                                m4::y_rotation( deg_to_rad( self.base_y_acc_rotation ).into() )
                            );

                            accumulated_rotations = m4::multiply_mat(
                                accumulated_rotations, 
                                m4::z_rotation( deg_to_rad( self.base_z_acc_rotation ).into() )
                            );

                            let accumulated_rotations_from_decomposed = (
                                r * Matrix4::from_column_slice(&accumulated_rotations)
                            )
                            .try_normalize(std::f32::EPSILON)
                            .unwrap_or_default();
                            
                            accumulated_rotations_for_normal_transformations[2] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());

                            let updated_model_matrix = m4::x_rotate_3_d(
                                original_transformation, 
                                accumulated_rotations 
                            );

                            ////////// animations above
                                                      
                            animation_models[2] = Some(updated_model_matrix);                         
                            animation_matrix_stack.drain(..); 
                        }
                    }
                }
            }  

            return (animation_models, accumulated_rotations_for_normal_transformations)

        // BACK LEGS ////////////////////////////////
        } else if let LegType::Back = self.position {
            for leg_part in 0..3 { // make it more semanthic
                
                if leg_part == 0 { // upper part

                    let clamping_model_matrix = m4::translate_3_d( 
                        *pre_matrix, // what happens with dereferencing a vector
                        m4::translation( 
                            self.body_clamp_point.0 - BACK_UPPER_LEG_WIDTH, 
                            self.body_clamp_point.1 - 0., 
                            self.body_clamp_point.2 - BACK_UPPER_LEG_DEPTH / 2., 
                        )
                    );

                    let original_transformation = self.get_back_leg_transformation(
                        clamping_model_matrix, 
                        leg_part, 
                        leg_i 
                    );

                    if !self.is_moving {

                        animation_models[0] = Some(original_transformation);
                        animation_matrix_stack.push(original_transformation);
                        accumulated_rotations_for_normal_transformations[0] = Some(m4::identity()); 

                    } else {
                        match direction {
                            Move::Forward => {
                                println!("move forward");

                                let displacement: f32;
                                if self.move_cycle == 1 {
                                    displacement = -1.;
                                } else {
                                    displacement = 1.;
                                }
                                
                                self.upper_z_acc_rotation += displacement; 
    
                                if !self.upper_forward_move_range.contains( &( self.upper_z_acc_rotation as f32 ) ) {
                                    self.switch_move_cycle();
                                }                    
                            },
                            Move::Left => {                                       
                                let rotation_y: f32;

                                if self.upper_side_y_move_cycle == 0 {
                                    rotation_y = 1.;
                                } else {
                                    rotation_y = -1.;
                                }
                                
                                self.upper_y_acc_rotation += rotation_y; // same values? 
     
                                if !self.upper_side_y_move_range.contains( &( self.upper_y_acc_rotation as f32 ) ) {
                                    self.switch_upper_side_y_move_cycle();
                                    
                                }                        
                            },
                            Move::Right => {                                     
                                let rotation_y: f32;

                                if self.upper_side_y_move_cycle == 0 {
                                    rotation_y = -1.;
                                } else {
                                    rotation_y = 1.;
                                }
                                
                                self.upper_y_acc_rotation += rotation_y; // same values? 
     
                                if !self.upper_side_y_move_range.contains( &( self.upper_y_acc_rotation as f32 ) ) {
                                    self.switch_upper_side_y_move_cycle();                                
                                }                  
                            },
                            Move::Jump => {
                                println!("move jump!");
                            },
                            Move::Static => {
                                println!("no move!");
                            },
                            Move::SpinDown => {
                                println!("move spindown!");
                            },
                            Move::SpinUp => {
                                println!("move spinup");
                            },
                            Move::ZoomOut => {
                                println!("move zoomout")
                            },
                            Move::ZoomIn => {
                                println!("move zoomin")
                            },
                        }
    
                        ////////// animations below
                        let mut updated_model_matrix = m4::translate_3_d( // here is the pivot point
                            original_transformation, 
                            m4::translation( 
                                BACK_UPPER_LEG_WIDTH, 
                                BACK_UPPER_LEG_BIG_HEIGHT / 2.,
                                BACK_UPPER_LEG_DEPTH / 2.
                            )
                        );
                        
                        let mut accumulated_rotations = m4::multiply_mat(
                        m4::z_rotation( deg_to_rad( self.upper_z_acc_rotation * -1. ).into() ), 
                        m4::y_rotation( deg_to_rad( self.upper_y_acc_rotation ).into() )
                        );

                        let accumulated_rotations_from_decomposed = (
                            r * Matrix4::from_column_slice(&accumulated_rotations)
                        )
                        .try_normalize(std::f32::EPSILON)
                        .unwrap_or_default();
                        
                        accumulated_rotations_for_normal_transformations[0] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());

                        updated_model_matrix = m4::multiply_mat(
                            updated_model_matrix, 
                            accumulated_rotations
                        );    

                        updated_model_matrix = m4::translate_3_d( // here is the pivot point
                            updated_model_matrix, 
                            m4::translation( 
                                - BACK_UPPER_LEG_WIDTH, 
                                (BACK_UPPER_LEG_BIG_HEIGHT / 2.) * -1.,
                                (BACK_UPPER_LEG_DEPTH / 2.) * -1.
                            )
                        );   
                        ////////// animations above
                     
                        animation_models[0] = Some(updated_model_matrix); 
                        animation_matrix_stack.push(updated_model_matrix); 
                    }
                } else {
        
                    let clamping_model_matrix = animation_matrix_stack.last().unwrap();

                    let original_transformation = self.get_back_leg_transformation(
                        *clamping_model_matrix, 
                        leg_part, 
                        leg_i 
                    );

                    if !self.is_moving {
                        animation_models[leg_part] = Some(original_transformation);
                        animation_matrix_stack.push(original_transformation);
                        accumulated_rotations_for_normal_transformations[leg_part] = Some(m4::identity()); 

                        if leg_part == 2 {
                            animation_matrix_stack.drain(..);
                        }
                    } else {
                        if leg_part == 1 { // joint
                            match direction {
                                Move::Forward => {
                                    println!("move forward");

                                    println!("what to do");
                                    let displacement: f32;
                                    if self.move_cycle == 1 {
                                        displacement = 1.5;
                                    } else {
                                        displacement = -1.5;
                                    }
                                    
                                    self.joint_z_acc_rotation += displacement; 

                                    log(&format!("updated z acc rotation: {:?} ", self.joint_z_acc_rotation));
                                },
                                Move::Left => {
                                    let rotation_x: f32;
                                    let rotation_y: f32;
                                
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = 1.;
                                        rotation_y = -1.;
                                    
                                    } else {
                                        rotation_x = -1.;
                                        rotation_y = 1.;
                                     
                                    }
                                
                                    self.joint_x_acc_rotation += rotation_x; 
                                    self.joint_y_acc_rotation += rotation_y;
                                },
                                Move::Right => {
                                    let rotation_x: f32;
                                    let rotation_y: f32;
                                                                      
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = -1.;
                                        rotation_y = 1.;                                  
                                    } else {
                                        rotation_x = 1.;
                                        rotation_y = -1.;
                                    }
                                    
                                    self.joint_x_acc_rotation += rotation_x; 
                                    self.joint_y_acc_rotation += rotation_y; 
                                },
                                Move::Jump => {
                                    println!("move jump!");
                                },
                                Move::Static => {
                                    println!("no move!");
                                },
                                Move::SpinDown => {
                                    println!("move spindown!");
                                },
                                Move::SpinUp => {
                                    println!("move spinup");
                                },
                                Move::ZoomOut => {
                                    println!("move zoomout")
                                },
                                Move::ZoomIn => {
                                    println!("move zoomin")
                                },
                            }
                            
                            ////////// animations below
                            let mut updated_model_matrix = m4::translate_3_d( 
                                original_transformation, 
                                m4::translation( 
                                    BACK_JOINT_LEG_WIDTH, 
                                    BACK_JOINT_LEG_BIG_HEIGHT / 2.,
                                    BACK_JOINT_LEG_DEPTH / 2.
                                )
                            );

                            let mut accumulated_rotations = m4::multiply_mat(
                                m4::z_rotation( deg_to_rad( self.joint_z_acc_rotation * -1. ).into() ), 
                                m4::x_rotation( deg_to_rad( self.joint_x_acc_rotation ).into() )
                            );
                            
                            accumulated_rotations = m4::multiply_mat(
                                accumulated_rotations, 
                                m4::y_rotation( deg_to_rad( self.joint_y_acc_rotation * -1. ).into() )
                            );

                            let accumulated_rotations_from_decomposed = (
                                r * Matrix4::from_column_slice(&accumulated_rotations)
                            )
                            .try_normalize(std::f32::EPSILON)
                            .unwrap_or_default();
                            
                            accumulated_rotations_for_normal_transformations[1] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());

                            //only side animations above
                            updated_model_matrix = m4::multiply_mat(
                                updated_model_matrix, 
                                accumulated_rotations
                            );

                            updated_model_matrix = m4::translate_3_d( 
                                updated_model_matrix, 
                                m4::translation( 
                                    - BACK_JOINT_LEG_WIDTH, 
                                    (BACK_JOINT_LEG_BIG_HEIGHT / 2.) * -1.,
                                    (BACK_JOINT_LEG_DEPTH / 2.) * -1.
                                )
                            );
                            ////////// animations above
                         
                            animation_models[1] = Some(updated_model_matrix);
                            animation_matrix_stack.push(updated_model_matrix); 
                        }

                        if leg_part == 2 { // base
                            match direction {
                                Move::Forward => {
                                    println!("move forward");
                                },
                                Move::Left => {
                                    let rotation_x: f32;
                                    let rotation_y: f32;    
                                    
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = -1.;
                                        rotation_y = -1.;
                                    } else {
                                        rotation_x = 1.;
                                        rotation_y = 1.;
                                    }
                                    
                                    self.base_x_acc_rotation += rotation_x;
                                    self.base_y_acc_rotation += rotation_y;              
                                },
                                Move::Right => {
                                    let rotation_x: f32;    
                                    let rotation_y: f32;    
                                    
                                    if self.upper_side_y_move_cycle == 0 {
                                        rotation_x = 1.;
                                        rotation_y = 1.;
                                    } else {
                                        rotation_x = -1.;
                                        rotation_y = -1.;
                                    }
                                    
                                    self.base_x_acc_rotation += rotation_x;  
                                    self.base_y_acc_rotation = rotation_y;
                                },
                                Move::Jump => {
                                    println!("move jump!");
                                },
                                Move::Static => {
                                    println!("no move!");
                                },
                                Move::SpinDown => {
                                    println!("move spindown!");
                                },
                                Move::SpinUp => {
                                    println!("move spinup");
                                },
                                Move::ZoomOut => {
                                    println!("move zoomout")
                                },
                                Move::ZoomIn => {
                                    println!("move zoomin")
                                },
                            }

                            ////////// animations below
                            
                            // only side animations below
                            let accumulated_rotations = m4::multiply_mat(
                                m4::x_rotation( deg_to_rad( self.base_x_acc_rotation ).into() ), 
                                m4::y_rotation( deg_to_rad( self.base_y_acc_rotation ).into() )
                            );

                            let accumulated_rotations_from_decomposed = (
                                r * Matrix4::from_column_slice(&accumulated_rotations)
                            )
                            .try_normalize(std::f32::EPSILON)
                            .unwrap_or_default();
                            
                            accumulated_rotations_for_normal_transformations[2] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());

                            // only side animations above
                            let updated_model_matrix = m4::multiply_mat(
                                original_transformation, 
                                accumulated_rotations
                            );

                            ////////// animations above
                                                      
                            animation_models[2] = Some(updated_model_matrix);                         
                            animation_matrix_stack.drain(..); 
                        }
                    }
                }
            } 
        } else { // MIDDLE LEGS (ELSE IF?)
            for leg_part in 0..3 { // make it more semanthic
                
                if leg_part == 0 { // upper part

                    let clamping_model_matrix = m4::translate_3_d( 
                        *pre_matrix, // what happens with dereferencing a vector
                        m4::translation( 
                            self.body_clamp_point.0 - 0., 
                            self.body_clamp_point.1 - 0., 
                            self.body_clamp_point.2 - 0., 
                        )
                    );

                    let original_transformation = self.get_middle_leg_transformation(
                        clamping_model_matrix, 
                        leg_part, 
                        leg_i 
                    );

                    if !self.is_moving {

                        animation_models[0] = Some(original_transformation);
                        animation_matrix_stack.push(original_transformation);
                        accumulated_rotations_for_normal_transformations[0] = Some(m4::identity()); 

                    } else {
                        match direction {
                            Move::Forward => {
                                let displacement: f32;
                                if self.move_cycle == 0 {
                                    displacement = 0.65;
                                } else {
                                    displacement = -0.65;
                                }
                                
                                self.upper_y_acc_rotation += displacement; 
                                self.upper_x_acc_rotation += displacement; 

                                if !self.upper_forward_move_range.contains( &( self.upper_y_acc_rotation as f32 ) ) {
                                    self.switch_move_cycle();
                                }      
                                
                            },
                            Move::Left => {
                                let rotation_z: f32;

                                if self.upper_side_z_move_cycle == 0 {
                                    rotation_z = -0.5;
                                } else {
                                    rotation_z = 0.5;
                                }

                                self.upper_z_acc_rotation += rotation_z;

                                if !self.upper_side_z_move_range.contains( &( self.upper_z_acc_rotation as f32 ) ) {
                                    self.switch_upper_side_z_move_cycle();
                                }     
                            },
                            Move::Right => {
                                let rotation_z: f32;

                                if self.upper_side_z_move_cycle == 0 {
                                    rotation_z = -0.5;
                                } else {
                                    rotation_z = 0.5;
                                }

                                self.upper_z_acc_rotation += rotation_z;
                                
                                if !self.upper_side_z_move_range.contains( &( self.upper_z_acc_rotation as f32 ) ) {
                                    self.switch_upper_side_z_move_cycle();
                                }     
                            },
                            Move::Jump => {
                                println!("move jump!");
                            },
                            Move::Static => {
                                println!("no move!");
                            },
                            Move::SpinDown => {
                                println!("move spindown!");
                            },
                            Move::SpinUp => {
                                println!("move spinup");
                            },
                            Move::ZoomOut => {
                                println!("move zoomout")
                            },
                            Move::ZoomIn => {
                                println!("move zoomin")
                            },
                        }
    
                        ////////// animations below
                        let mut updated_model_matrix = m4::translate_3_d( 
                            original_transformation, 
                            m4::translation( 
                                MIDDLE_JOINT_LEG_WIDTH, 
                                MIDDLE_JOINT_LEG_BIG_HEIGHT / 2.,
                                MIDDLE_JOINT_LEG_DEPTH / 2.
                            )
                        );

                        let mut accumulated_rotations = m4::multiply_mat(
                            m4::y_rotation( deg_to_rad( self.upper_y_acc_rotation ).into() ), 
                            m4::x_rotation( deg_to_rad( self.upper_x_acc_rotation ).into() )
                        );

                        accumulated_rotations = m4::multiply_mat(
                            accumulated_rotations, 
                            m4::z_rotation( deg_to_rad( self.upper_z_acc_rotation ).into() )
                        );

                        let accumulated_rotations_from_decomposed = (
                            r * Matrix4::from_column_slice(&accumulated_rotations)
                        )
                        .try_normalize(std::f32::EPSILON)
                        .unwrap_or_default();
                        
                        accumulated_rotations_for_normal_transformations[0] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());
        
                        // only side animation above
                        updated_model_matrix = m4::multiply_mat(
                            updated_model_matrix, 
                            accumulated_rotations
                        ); 

                        updated_model_matrix = m4::translate_3_d( 
                            updated_model_matrix, 
                            m4::translation( 
                                - MIDDLE_JOINT_LEG_WIDTH, 
                                (MIDDLE_JOINT_LEG_BIG_HEIGHT / 2.) * -1.,
                                (MIDDLE_JOINT_LEG_DEPTH / 2.) * -1.
                            )
                        );
                        ////////// animations above
                     
                        animation_models[0] = Some(updated_model_matrix); 
                        animation_matrix_stack.push(updated_model_matrix); 
                    }
                } else {
        
                    let clamping_model_matrix = animation_matrix_stack.last().unwrap();

                    let original_transformation = self.get_middle_leg_transformation(
                        *clamping_model_matrix, 
                        leg_part, 
                        leg_i 
                    );

                    if !self.is_moving {
                        animation_models[leg_part] = Some(original_transformation);
                        animation_matrix_stack.push(original_transformation); 
                        accumulated_rotations_for_normal_transformations[leg_part] = Some(m4::identity());

                        if leg_part == 2 {
                            animation_matrix_stack.drain(..);
                        }
                    } else {
                        if leg_part == 1 { // joint
                            match direction {
                                Move::Forward => {
                                    println!("move forward");
                                },
                                Move::Left => {
                                    self.joint_z_acc_rotation = self.upper_z_acc_rotation * -1.;
                                },
                                Move::Right => {
                                    self.joint_z_acc_rotation = self.upper_z_acc_rotation * -1.;
                                },
                                Move::Jump => {
                                    println!("move jump!");
                                },
                                Move::Static => {
                                    println!("no move!");
                                },
                                Move::SpinDown => {
                                    println!("move spindown!");
                                },
                                Move::SpinUp => {
                                    println!("move spinup");
                                },
                                Move::ZoomOut => {
                                    println!("move zoomout")
                                },
                                Move::ZoomIn => {
                                    println!("move zoomin")
                                },
                            }
                            
                            ////////// animations below
                            
                            // only side animations below
                            let mut updated_model_matrix = m4::translate_3_d(
                                original_transformation, 
                                m4::translation(
                                    MIDDLE_JOINT_LEG_WIDTH, 
                                    MIDDLE_JOINT_LEG_BIG_HEIGHT / 2., 
                                    MIDDLE_JOINT_LEG_DEPTH / 2.
                                )
                            );

                            let accumulated_rotations = m4::multiply_mat(
                                m4::identity(),
                                m4::z_rotation( deg_to_rad( self.joint_z_acc_rotation ).into() ), 
                            );

                            let accumulated_rotations_from_decomposed = (
                                r * Matrix4::from_column_slice(&accumulated_rotations)
                            )
                            .try_normalize(std::f32::EPSILON)
                            .unwrap_or_default();
                            
                            accumulated_rotations_for_normal_transformations[1] = Some(accumulated_rotations_from_decomposed.as_slice().try_into().unwrap());

                            updated_model_matrix = m4::multiply_mat(
                                updated_model_matrix, 
                                accumulated_rotations 
                            );

                            updated_model_matrix = m4::translate_3_d(
                                updated_model_matrix, 
                                m4::translation(
                                    - MIDDLE_JOINT_LEG_WIDTH, 
                                    (MIDDLE_JOINT_LEG_BIG_HEIGHT / 2.) * -1., 
                                    (MIDDLE_JOINT_LEG_DEPTH / 2.) * -1.
                                )
                            );
                            // only side animation above

                            ////////// animations above
                         
                            animation_models[1] = Some(updated_model_matrix);
                            animation_matrix_stack.push(updated_model_matrix); 
                        }

                        if leg_part == 2 { // base
                            match direction {
                                Move::Forward => {
                                    println!("move forward");
                                },
                                Move::Left => {
                                    println!("move left!");
                                },
                                Move::Right => {
                                    println!("move right!");
                                },
                                Move::Jump => {
                                    println!("move jump!");
                                },
                                Move::Static => {
                                    println!("no move!");
                                },
                                Move::SpinDown => {
                                    println!("move spindown!");
                                },
                                Move::SpinUp => {
                                    println!("move spinup");
                                },
                                Move::ZoomOut => {
                                    println!("move zoomout")
                                },
                                Move::ZoomIn => {
                                    println!("move zoomin")
                                },
                            }

                            let accumulated_rotations = m4::multiply_mat(
                                m4::identity(), 
                                r.
                                try_normalize(std::f32::EPSILON)
                                .unwrap_or_default()
                                .as_slice()
                                .try_into()
                                .unwrap()
                            );
                            
                            accumulated_rotations_for_normal_transformations[2] = Some( accumulated_rotations );
                                                           
                            animation_models[2] = Some(original_transformation);                         
                            animation_matrix_stack.drain(..); 
                        }
                    }
                }
            } 
            
            return (animation_models, accumulated_rotations_for_normal_transformations)
        }       
            
        (animation_models, accumulated_rotations_for_normal_transformations)
    }

    
    // 0 = upper   -   0 = left
    // 1 = joint   -   1 = right
    // 2 = base
    // in case of setting upper leg dont forget to send the transformed pivot point
    pub fn get_back_leg_transformation(&self, pre_matrix: [f32; 16], leg_part: usize, leg_i: usize) -> [f32; 16] {
        
        if leg_part == 0 {
            let mut upper_leg_model_matrix = m4::x_rotate_3_d(
                pre_matrix, 
                m4::x_rotation( deg_to_rad( 180. ).into() )
            );
            
            upper_leg_model_matrix = m4::translate_3_d( 
                upper_leg_model_matrix, 
                m4::translation( 
                    BACK_UPPER_LEG_WIDTH, 
                    0.,
                    - BACK_UPPER_LEG_DEPTH
                )
            );
            
            let mut back_body_convergent_angle = get_back_body_convergent_angle() as f64;
            if leg_i == 0 {
                back_body_convergent_angle *= -1.;
            }
            
            upper_leg_model_matrix = m4::translate_3_d( 
                upper_leg_model_matrix, 
                m4::translation( 
                    0., 
                    - BACK_UPPER_LEG_SMALL_HEIGHT,
                    BACK_UPPER_LEG_DEPTH / 2.
                )
            );
            
            upper_leg_model_matrix = m4::y_rotate_3_d(
                upper_leg_model_matrix, 
                m4::y_rotation( back_body_convergent_angle )
            );
            
            upper_leg_model_matrix = m4::translate_3_d( 
                upper_leg_model_matrix, 
                m4::translation( 
                    0., 
                    BACK_UPPER_LEG_SMALL_HEIGHT,
                    - BACK_UPPER_LEG_DEPTH / 2.
                )
            );
            
            upper_leg_model_matrix = m4::z_rotate_3_d(
                upper_leg_model_matrix, 
                m4::z_rotation( deg_to_rad( 50. ).into() )
            );
            
            upper_leg_model_matrix = m4::translate_3_d( 
                upper_leg_model_matrix, 
                m4::translation( 
                    - BACK_UPPER_LEG_WIDTH, 
                    0.,
                    0.
                )
            );
       
            return upper_leg_model_matrix
        } else if leg_part == 1 {
                
            let mut joint_leg_model_matrix = m4::z_rotate_3_d(
                pre_matrix, 
                m4::z_rotation(deg_to_rad(-100.).into())
            );
                
            joint_leg_model_matrix = m4::translate_3_d( 
                joint_leg_model_matrix, 
                m4::translation(
                //FRONTAL_JOINT_LEG_WIDTH * -1., 
                BACK_JOINT_LEG_WIDTH * -1., 
                FRONTAL_UPPER_LEG_BIG_HEIGHT / 2. - FRONTAL_JOINT_LEG_BIG_HEIGHT / 2., 
                BACK_UPPER_LEG_DEPTH / 2. - BACK_JOINT_LEG_DEPTH / 2. 
                )
            );       

            return joint_leg_model_matrix
        } else {     
    
            let mut base_leg_model_matrix = m4::z_rotate_3_d(
                pre_matrix, 
                m4::z_rotation( deg_to_rad(163.).into() )
            );

            base_leg_model_matrix = m4::translate_3_d(
                base_leg_model_matrix, 
                m4::translation(
                0.,
                (BACK_BASE_LEG_HEIGHT / 2. + BACK_JOINT_LEG_BIG_HEIGHT / 2.) * -1.,
                BACK_BASE_LEG_DEPTH / 2.
                )
            );    
            

            return base_leg_model_matrix
        }
    }


    // 0 = upper   -   0 = left
    // 1 = joint   -   1 = right
    // 2 = base
    // in case of setting upper leg dont forget to send the transformed pivot point
    pub fn get_middle_leg_transformation(&self, pre_matrix: [f32; 16], leg_part: usize, leg_i: usize) -> [f32; 16] {
        if leg_part == 0 {
            let mut upper_leg_model_matrix = m4::translate_3_d( 
                pre_matrix, 
                m4::identity()
            );

            if leg_i < 2 { // side I

                upper_leg_model_matrix = m4::y_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::y_rotation( deg_to_rad( -90. ).into() )
                );
                

                upper_leg_model_matrix = m4::x_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::x_rotation( deg_to_rad( -180. ).into() ) // 180 invert + 55 up
                );

                upper_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad( 55. ).into() ) 
                );

            
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH + 2., 
                        //MIDDLE_UPPER_LEG_BIG_HEIGHT - MIDDLE_UPPER_LEG_SMALL_HEIGHT,
                        0.,
                        (MIDDLE_UPPER_LEG_DEPTH / 2.) * -1.
                    )
                );

            } else { // side II
            
                upper_leg_model_matrix = m4::y_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::y_rotation( deg_to_rad( 90. ).into() )
                );

                upper_leg_model_matrix = m4::x_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::x_rotation( deg_to_rad( -180. ).into() ) // 180 invert + 55 up
                );

                upper_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad( 55.).into() ) 
                );

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH + 2., 
                        0.,
                        (MIDDLE_UPPER_LEG_DEPTH / 2.) * -1.
                    )
                );
            }
            
            return upper_leg_model_matrix
        
        } else if leg_part == 1 {
            let mut joint_leg_model_matrix = m4::z_rotate_3_d(
                pre_matrix, 
                m4::z_rotation(deg_to_rad(-95.).into())
            );
            
            joint_leg_model_matrix = m4::translate_3_d( 
                joint_leg_model_matrix, 
                m4::translation(
                MIDDLE_JOINT_LEG_WIDTH * -1., 
                MIDDLE_UPPER_LEG_BIG_HEIGHT / 2. - MIDDLE_JOINT_LEG_BIG_HEIGHT / 2., 
                MIDDLE_UPPER_LEG_DEPTH / 2. - MIDDLE_JOINT_LEG_DEPTH / 2. 
                )
            );

            return joint_leg_model_matrix
            
        } else {
            // base leg transformations
            let mut base_leg_model_matrix = m4::z_rotate_3_d(
                pre_matrix, 
                m4::z_rotation( deg_to_rad(163.).into() )
            );    
            base_leg_model_matrix = m4::translate_3_d(
                base_leg_model_matrix, 
                m4::translation(
                0.,
                ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
                (MIDDLE_JOINT_LEG_DEPTH / 2.) - (MIDDLE_BASE_LEG_DEPTH / 2.), 
                )
            );

            return base_leg_model_matrix

        }
    }

    // 0 = upper   -   0 = left
    // 1 = joint   -   1 = right
    // 2 = base
    // in case of setting upper leg dont forget to send the transformed pivot point
    pub fn get_frontal_leg_transformation(&self, pre_matrix: [f32; 16], leg_part: usize, leg_i: usize) -> [f32; 16] {
        
        if leg_part == 0 { // upper
            let mut frontal_body_convergent_angle = get_frontal_body_convergent_angle() as f64;

            if leg_i == 1 {
                frontal_body_convergent_angle *= -1.;
            }

            let mut upper_leg_model_matrix = m4::y_rotate_3_d(
                pre_matrix, 
                m4::y_rotation( frontal_body_convergent_angle )
            );

            upper_leg_model_matrix = m4::z_rotate_3_d(
                upper_leg_model_matrix, 
                m4::z_rotation( deg_to_rad( 55. ).into() ) 
            );

            upper_leg_model_matrix = m4::z_rotate_3_d(
                upper_leg_model_matrix, 
                m4::z_rotation( deg_to_rad( 180. ).into() ) 
            );

            upper_leg_model_matrix = m4::translate_3_d( // here is the pivot point
                upper_leg_model_matrix, 
                m4::translation( 
                    - FRONTAL_UPPER_LEG_WIDTH, 
                    FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.,
                    - FRONTAL_UPPER_LEG_DEPTH / 2.
                )
            );

            return upper_leg_model_matrix
        } else if leg_part == 1 { // joint
            let mut joint_leg_model_matrix = m4::z_rotate_3_d(
                pre_matrix, 
                m4::z_rotation(deg_to_rad(-102.).into())
            );
            
            joint_leg_model_matrix = m4::translate_3_d( 
                joint_leg_model_matrix, 
                m4::translation(
                FRONTAL_JOINT_LEG_WIDTH * -1., 
                FRONTAL_UPPER_LEG_BIG_HEIGHT / 2. - FRONTAL_JOINT_LEG_BIG_HEIGHT / 2., 
                FRONTAL_UPPER_LEG_DEPTH / 2. - FRONTAL_JOINT_LEG_DEPTH / 2. 
                )
            );

            return joint_leg_model_matrix;

        } else { // base (leg_part == 2) 
            let mut base_leg_model_matrix = m4::z_rotate_3_d(
                pre_matrix, 
                m4::z_rotation( deg_to_rad(163.).into() )
            );    

            base_leg_model_matrix = m4::translate_3_d(
                base_leg_model_matrix, 
                m4::translation(
                0.,
                ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
                (FRONTAL_JOINT_LEG_DEPTH / 2.) - (FRONTAL_BASE_LEG_DEPTH / 2.), 
                )
            );

            return base_leg_model_matrix;
        }
    }

}


