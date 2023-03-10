use std::ops::Range;
use web_sys::HtmlCanvasElement;

use crate::{spider::LegType, data_structures::*, modules::m4::m4::M4 as m4, constants::*, webgl_utils::deg_to_rad, setup_ui_control::{SpiderControl, Move}, matrix_stack::MatrixStack, log};


pub struct Leg {
    pub position: LegType,
    pub upper_move_range: Range<f32>,
    pub joint_move_range: Range<f32>,
    pub base_move_range: Range<f32>,
    pub vertex_data: Vec<Vec<f32>>,
    pub upper_z_acc_rotation: f32,
    pub joint_z_acc_rotation: f32,
    pub base_z_acc_rotation: f32,
    pub joint_acc_translation: f32,
    pub body_clamp_point: (f32, f32, f32),
    move_cycle: u8,
    pub is_moving: bool
}

impl Leg {
    pub fn new(leg_position: LegType, body_clamp_point: (f32, f32, f32), move_cycle: u8) -> Self {
        
        // upper, joint and base
        let vertex_data = get_leg_data(&leg_position);
    
        Self {
            vertex_data,
            body_clamp_point, 
            position: leg_position,
            upper_move_range: Range { start: -24.5, end: 5. },
            joint_move_range: Range { start: 0., end: 17. },
            base_move_range: Range { start: 0., end: 11. },
            upper_z_acc_rotation: 0.,
            joint_z_acc_rotation: 0.,
            base_z_acc_rotation: 0.,
            joint_acc_translation: 0.,
            move_cycle,
            is_moving: false
        }
    }

    pub fn stop_moving(&mut self) {
        self.is_moving = false;
    }

    pub fn start_moving(&mut self) {
        self.is_moving = true;
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
        animation_matrix_stack_two: &mut MatrixStack // what is that for?
    ) -> [Option<[f32; 16]>; 3] {

        //let mut animation_model_matrices = Vec::new();
        let mut animation_models: [Option<[f32; 16]>; 3] = [None; 3];

        let mut animation_matrix_stack = Vec::new();
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

                                log(&format!("updated z acc rotation: {:?} ", self.upper_z_acc_rotation)); 
    
                                if !self.upper_move_range.contains( &( self.upper_z_acc_rotation as f32 ) ) {
                                    self.switch_move_cycle();
                                }                     
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
        
                        updated_model_matrix = m4::z_rotate_3_d(
                            updated_model_matrix, 
                            m4::z_rotation( deg_to_rad( self.upper_z_acc_rotation ).into() ) 
                        );
        
                        updated_model_matrix = m4::translate_3_d( // here is the pivot point
                            updated_model_matrix, 
                            m4::translation( 
                                - FRONTAL_UPPER_LEG_WIDTH, 
                                (FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.) * -1.,
                                (FRONTAL_UPPER_LEG_DEPTH / 2.) * -1.
                            )
                        );   
                        ////////// animations above
                     
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

                        if leg_part == 2 {
                            animation_matrix_stack.drain(..);
                        }
                    } else {
                        if leg_part == 1 { // joint
                            match direction {
                                Move::Forward => {
                                    println!("what to do");
                                    let displacement: f32;
                                    if self.move_cycle == 0 {
                                        displacement = -1.5;
                                    } else {
                                        displacement = 1.5;
                                    }
                                    
                                    self.joint_z_acc_rotation += displacement; 

                                    log(&format!("updated z acc rotation: {:?} ", self.joint_z_acc_rotation));         
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
                            }
                            
                            ////////// animations
                            let mut updated_model_matrix = m4::translate_3_d( 
                                original_transformation, 
                                m4::translation( 
                                    FRONTAL_JOINT_LEG_WIDTH, 
                                    FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.,
                                    FRONTAL_JOINT_LEG_DEPTH / 2.
                                )
                            );
            
                            updated_model_matrix = m4::z_rotate_3_d(
                                updated_model_matrix, 
                                m4::z_rotation( deg_to_rad( self.joint_z_acc_rotation * -1. ).into() ) 
                            );
            
                            updated_model_matrix = m4::translate_3_d( 
                                updated_model_matrix, 
                                m4::translation( 
                                    - FRONTAL_JOINT_LEG_WIDTH, 
                                    (FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) * -1.,
                                    (FRONTAL_JOINT_LEG_DEPTH / 2.) * -1.
                                )
                            );
                            ////////// animations
                         
                            animation_models[1] = Some(updated_model_matrix);
                            animation_matrix_stack.push(updated_model_matrix); 
                        }

                        if leg_part == 2 { // base
                            match direction {
                                Move::Forward => {
                                    let displacement: f32;
                                    if self.move_cycle == 0 {
                                        if self.base_z_acc_rotation < 0. {
                                            displacement = 1.;
                                        } else {
                                            displacement = 0.;
                                        }
                                    } else {
                                        displacement = -1.;
                                    }
                                    
                                    self.base_z_acc_rotation += displacement;  
                                         
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
                            }

                            ////////// animations below
                                  
                            let updated_model_matrix = m4::z_rotate_3_d(
                                original_transformation, 
                                m4::z_rotation( deg_to_rad( self.base_z_acc_rotation ).into() ) 
                            );
        
                            ////////// animations above
                                                      
                            animation_models[2] = Some(updated_model_matrix);                         
                            animation_matrix_stack.drain(..); 
                        }
                    }
                }
            }  

            return animation_models

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
                                
                                log(&format!("[BACK LEGS]: {:?} ", self.upper_z_acc_rotation)); 
    
                                if !self.upper_move_range.contains( &( self.upper_z_acc_rotation as f32 ) ) {
                                    self.switch_move_cycle();
                                }                    
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
        
                        updated_model_matrix = m4::z_rotate_3_d(
                            updated_model_matrix, 
                            m4::z_rotation( deg_to_rad( self.upper_z_acc_rotation * -1. ).into() ) 
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
            
                            updated_model_matrix = m4::z_rotate_3_d(
                                updated_model_matrix, 
                                m4::z_rotation( deg_to_rad( self.joint_z_acc_rotation * -1. ).into() ) 
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
                            }

                            ////////// animations below
                            
                    
                            ////////// animations above
                                                      
                            animation_models[2] = Some(original_transformation);                         
                            animation_matrix_stack.drain(..); 
                        }
                    }
                }
            } 
        } else { // middle legs
            for leg_part in 0..3 { // make it more semanthic
                
                if leg_part == 0 { // upper part

                    let clamping_model_matrix = m4::translate_3_d( 
                        *pre_matrix, // what happens with dereferencing a vector
                        m4::translation( 
                            // self.body_clamp_point.0 - BACK_UPPER_LEG_WIDTH, 
                            // self.body_clamp_point.1 - 0., 
                            // self.body_clamp_point.2 - BACK_UPPER_LEG_DEPTH / 2., 
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

                    } else {
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
                        }
    
                        ////////// animations below
                       
                        ////////// animations above
                     
                        animation_models[0] = Some(original_transformation); 
                        animation_matrix_stack.push(original_transformation); 
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
                            }
                            
                            ////////// animations below
                            
                            ////////// animations above
                         
                            animation_models[1] = Some(original_transformation);
                            animation_matrix_stack.push(original_transformation); 
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
                            }

                            ////////// animations below
                            
                    
                            ////////// animations above
                                                      
                            animation_models[2] = Some(original_transformation);                         
                            animation_matrix_stack.drain(..); 
                        }
                    }
                }
            } 
            
            return animation_models
        }       
            
        animation_models
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
                    FRONTAL_UPPER_LEG_WIDTH, 
                    0.,
                    - FRONTAL_UPPER_LEG_DEPTH
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
                    - FRONTAL_UPPER_LEG_SMALL_HEIGHT,
                    FRONTAL_UPPER_LEG_DEPTH / 2.
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
                    FRONTAL_UPPER_LEG_SMALL_HEIGHT,
                    - FRONTAL_UPPER_LEG_DEPTH / 2.
                )
            );
            
            upper_leg_model_matrix = m4::z_rotate_3_d(
                upper_leg_model_matrix, 
                m4::z_rotation( deg_to_rad( 35. ).into() )
            );
            
            upper_leg_model_matrix = m4::translate_3_d( 
                upper_leg_model_matrix, 
                m4::translation( 
                    - FRONTAL_UPPER_LEG_WIDTH, 
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
                FRONTAL_JOINT_LEG_WIDTH * -1., 
                FRONTAL_UPPER_LEG_BIG_HEIGHT / 2. - FRONTAL_JOINT_LEG_BIG_HEIGHT / 2., 
                BACK_UPPER_LEG_DEPTH / 2. - BACK_JOINT_LEG_DEPTH / 2. 
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
                (BACK_JOINT_LEG_DEPTH / 2.) - (FRONTAL_BASE_LEG_DEPTH / 2.), 
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

                if leg_i == 0 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( -7. ).into() ) // 180 invert + 55 up
                    );
                }

                if leg_i == 1 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( 7. ).into() ) // 180 invert + 55 up
                    );
                }

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
               
                // upper_leg_model_matrix = m4::translate_3_d( 
                //     upper_leg_model_matrix, 
                //     m4::translation( 
                //         MIDDLE_UPPER_LEG_WIDTH,
                //         0.,
                //         MIDDLE_UPPER_LEG_DEPTH / 2.
                //     )
                // );

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

                if leg_i == 2 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( 7. ).into() ) // 180 invert + 55 up
                    );
                }

                if leg_i == 3 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( -7. ).into() ) // 180 invert + 55 up
                    );
                }

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH + 2., 
                        //MIDDLE_UPPER_LEG_BIG_HEIGHT - MIDDLE_UPPER_LEG_SMALL_HEIGHT,
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


