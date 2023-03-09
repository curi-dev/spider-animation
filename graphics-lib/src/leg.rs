use std::ops::Range;
use web_sys::HtmlCanvasElement;

use crate::{spider::LegType, data_structures::*, modules::m4::m4::M4 as m4, constants::*, webgl_utils::deg_to_rad, setup_ui_control::{SpiderControl, Move}, matrix_stack::MatrixStack};


pub struct Leg {
    pub position: LegType,
    pub upper_move_range: Range<f32>,
    pub joint_move_range: Range<f32>,
    pub base_move_range: Range<f32>,
    pub vertex_data: Vec<Vec<f32>>,
    pub upper_and_joint_z_acc_rotation: f32,
    pub base_z_acc_rotation: f32,
    pub joint_acc_translation: f32,
    pub body_clamp_point: (f32, f32, f32)
}

impl Leg {
    pub fn new(leg_position: LegType, body_clamp_point: (f32, f32, f32)) -> Self {
        // upper, joint and base
        let vertex_data = get_leg_data(&leg_position);
    
        Self {
            vertex_data,
            body_clamp_point, // what type of translation and is accumulated?
            position: leg_position,
            upper_move_range: Range { start: 0., end: 11. },
            joint_move_range: Range { start: 0., end: 11. },
            base_move_range: Range { start: 0., end: 11. },
            upper_and_joint_z_acc_rotation: 0.,
            base_z_acc_rotation: 0.,
            joint_acc_translation: 0.,
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
                     // save the original transformation for upper leg
                    //animation_matrix_stack.push(original_transformation);

                    match direction {
                        Move::Forward => {
                            self.upper_and_joint_z_acc_rotation -= 1.;
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
                        m4::z_rotation( deg_to_rad( self.upper_and_joint_z_acc_rotation ).into() ) 
                    );
    
                    updated_model_matrix = m4::translate_3_d( // here is the pivot point
                        updated_model_matrix, 
                        m4::translation( 
                            - FRONTAL_UPPER_LEG_WIDTH, 
                            (FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.) * -1.,
                            (FRONTAL_UPPER_LEG_DEPTH / 2.) * -1.
                        )
                    );

                    //animation_model_matrices.push(updated_model_matrix);
                    animation_models[0] = Some(updated_model_matrix); 
                    animation_matrix_stack.push(updated_model_matrix); 

                } else {
                    //let clamping_model_matrix = *animation_matrix_stack.stack.last().unwrap();
                    let clamping_model_matrix = animation_matrix_stack.last().unwrap();

                    let original_transformation = self.get_frontal_leg_transformation(
                        *clamping_model_matrix, 
                        leg_part, 
                        leg_i 
                    );

                    // animation_matrix_stack.push(original_transformation);
                    
                    if leg_part == 1 { // joint
                        match direction {
                            Move::Forward => {
                                println!("move forward!");
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
                        
                        // animations
                        let updated_model_matrix = m4::translate_3_d( // here is the pivot point
                            original_transformation, 
                            m4::identity()
                        );
    
                        // animations
                        //animation_model_matrices.push(updated_model_matrix);  
                        animation_models[1] = Some(updated_model_matrix);
                        animation_matrix_stack.push(updated_model_matrix); 
                    }
        
                    if leg_part == 2 { // base
                        match direction {
                            Move::Forward => {
                                println!("move forward!");
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
                        
                        // animations
                        let updated_model_matrix = m4::translate_3_d( // here is the pivot point
                            original_transformation, 
                            m4::identity()
                        );
    
                        // animations
                        //animation_model_matrices.push(updated_model_matrix); 
                        animation_models[2] = Some(updated_model_matrix);  
                        
                        //animation_matrix_stack.reset(); // drain all values
                        animation_matrix_stack.drain(..); // drain all values
                    }
                }
            }  

            //return animation_model_matrices.try_into().unwrap(); 
            return animation_models
        } else {
            animation_models[0] = Some(m4::identity());  
            animation_models[1] = Some(m4::identity());  
            animation_models[1] = Some(m4::identity());  
            
            return animation_models
        }       
            

    }

    // pub fn walk_animate(
    //     &mut self, 
    //     pre_matrix: &[f32; 16], 
    //     direction: &Move, 
    //     leg_i: usize,
    //     animation_matrix_stack: &mut MatrixStack
    // ) -> [[f32; 16]; 3] {
    //     let original_transformations = self.get_original_transformations(pre_matrix, leg_i);
        
    //     let mut animated_model_matrix = Vec::new();

    //     for (i, transformation_model_matrix) in original_transformations.iter().enumerate() {           
     
    //         if i == 0 { // upper
    //             match direction {
    //                 Move::Forward => {
    //                     self.upper_and_joint_z_acc_rotation -= 1.;
    //                 },
    //                 Move::Left => {
    //                     println!("move left!");
    //                 },
    //                 Move::Right => {
    //                     println!("move right!");
    //                 },
    //                 Move::Jump => {
    //                     println!("move jump!");
    //                 },
    //                 Move::Static => {
    //                     println!("no move!");
    //                 },
    //             }
                
    //             let angle = self.upper_and_joint_z_acc_rotation;

    //             let mut updated_model_matrix = m4::translate_3_d( // here is the pivot point
    //                 *transformation_model_matrix, 
    //                 m4::translation( 
    //                     FRONTAL_UPPER_LEG_WIDTH, 
    //                     FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.,
    //                     FRONTAL_UPPER_LEG_DEPTH / 2.
    //                 )
    //             );

    //             updated_model_matrix = m4::z_rotate_3_d(
    //                 updated_model_matrix, 
    //                 m4::z_rotation( deg_to_rad( angle ).into() ) 
    //             );

    //             updated_model_matrix = m4::translate_3_d( // here is the pivot point
    //                 updated_model_matrix, 
    //                 m4::translation( 
    //                     - FRONTAL_UPPER_LEG_WIDTH, 
    //                     (FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.) * -1.,
    //                     (FRONTAL_UPPER_LEG_DEPTH / 2.) * -1.
    //                 )
    //             );

    //             animation_matrix_stack.push(updated_model_matrix.clone());  
    //             animated_model_matrix.push(updated_model_matrix);              
    //         }
            
    //         if i == 1 { // joint
    //             match direction {
    //                 Move::Forward => {
    //                     println!("move forward!");
    //                 },
    //                 Move::Left => {
    //                     println!("move left!");
    //                 },
    //                 Move::Right => {
    //                     println!("move right!");
    //                 },
    //                 Move::Jump => {
    //                     println!("move jump!");
    //                 },
    //                 Move::Static => {
    //                     println!("no move!");
    //                 },
    //             }
                
    //             //let angle = self.upper_and_joint_z_acc_rotation * -1.;
    //             let original_transformation_model_matrix = self.get_original_transformations(
    //                 animation_matrix_stack.stack.last().unwrap(), 
    //                 leg_i
    //             );
                
    //             let updated_model_matrix = m4::translate_3_d( // here is the pivot point
    //                 original_transformation_model_matrix[1], 
    //                 m4::identity()
    //             );
                
    //             animation_matrix_stack.push(updated_model_matrix.clone());  
    //             animated_model_matrix.push(updated_model_matrix);              
    //         }

    //         if i == 2 { // base
    //             match direction {
    //                 Move::Forward => {
    //                     println!("move forward!");
    //                 },
    //                 Move::Left => {
    //                     println!("move left!");
    //                 },
    //                 Move::Right => {
    //                     println!("move right!");
    //                 },
    //                 Move::Jump => {
    //                     println!("move jump!");
    //                 },
    //                 Move::Static => {
    //                     println!("no move!");
    //                 },
    //             }
                
    //             //let angle = self.base_z_acc_rotation;

    //             let original_transformation_model_matrix = self.get_original_transformations(
    //                 animation_matrix_stack.stack.last().unwrap(), 
    //                 leg_i
    //             );

    //             let updated_model_matrix = m4::translate_3_d( // here is the pivot point
    //                 original_transformation_model_matrix[2], 
    //                 m4::identity()
    //             );
    
    //             animated_model_matrix.push(updated_model_matrix);
    //         }
            
    //     };
                   
    //     animated_model_matrix.try_into().unwrap()

    // }

    pub fn get_back_leg_transformation(&self, pre_matrix: [f32; 16], leg_part: usize, leg_i: usize) -> [f32; 16] {
        if leg_part == 0 {
            let mut upper_leg_model_matrix = m4::x_rotate_3_d(
                pre_matrix, 
                m4::x_rotation( deg_to_rad( 180. ).into() )
            );

            upper_leg_model_matrix = m4::translate_3_d( 
                upper_leg_model_matrix, // clamp matrix
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
                upper_leg_model_matrix, // clamp matrix
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
                upper_leg_model_matrix, // clamp matrix
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
                upper_leg_model_matrix, // clamp matrix
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
                m4::z_rotation(deg_to_rad(-95.).into())
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

            return base_leg_model_matrix;
        }
    }

    pub fn get_middle_leg_transformation(&self, pre_matrix: [f32; 16], leg_part: usize, leg_i: usize) -> [f32; 16] {
        if leg_part == 0 {
            let mut upper_leg_model_matrix = m4::translate_3_d( 
                pre_matrix, 
                m4::identity()
            );

            if leg_i < 3 {
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        MIDDLE_UPPER_LEG_WIDTH,
                        (MIDDLE_UPPER_LEG_BIG_HEIGHT - MIDDLE_UPPER_LEG_SMALL_HEIGHT) * -1.,
                        MIDDLE_UPPER_LEG_DEPTH / 2.,
                    )
                );

                upper_leg_model_matrix = m4::y_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::y_rotation( deg_to_rad( -90. ).into() )
                );
               
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH, 
                        MIDDLE_UPPER_LEG_BIG_HEIGHT - MIDDLE_UPPER_LEG_SMALL_HEIGHT,
                        ( MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.
                    )
                );

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        MIDDLE_UPPER_LEG_WIDTH,
                        0.,
                        MIDDLE_UPPER_LEG_DEPTH / 2.,
                    )
                );
        
                upper_leg_model_matrix = m4::x_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::x_rotation( deg_to_rad(-180.).into() ) // 180 invert + 55 up
                );

                upper_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad(55.).into() ) // 180 invert + 55 up
                );

                // rotate the end legs in 15 degrees 
                if leg_i == 0 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( -7. ).into() ) // 180 invert + 55 up
                    );
                }

                if leg_i == 2 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( 7. ).into() ) // 180 invert + 55 up
                    );
                }
        
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH,
                        0.,
                        (MIDDLE_UPPER_LEG_DEPTH / 2.) * -1.,
                    )
                );
            } else {
                // other side legs
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        MIDDLE_UPPER_LEG_WIDTH,
                        0.,
                        MIDDLE_UPPER_LEG_DEPTH / 2.
                    )
                );

                upper_leg_model_matrix = m4::y_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::y_rotation( deg_to_rad( 90. ).into() )
                );

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH, 
                        0.,
                         (MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.,
                    )
                );

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH,
                        0.,
                        MIDDLE_UPPER_LEG_DEPTH / 2.,
                    )
                );

                upper_leg_model_matrix = m4::x_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::x_rotation( deg_to_rad( -180. ).into() ) // 180 invert + 55 up
                );
                
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        MIDDLE_UPPER_LEG_WIDTH,
                        0.,
                        ( MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.,
                    )
                );

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        MIDDLE_UPPER_LEG_WIDTH,
                        0.,
                        MIDDLE_UPPER_LEG_DEPTH / 2.,
                    )
                );

                upper_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad( 55.).into() ) 
                );

                // rotate the end legs in 15 degrees 
                if leg_i == 3 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( 7. ).into() ) // 180 invert + 55 up
                    );
                }

                if leg_i == 5 {
                    upper_leg_model_matrix = m4::y_rotate_3_d(
                        upper_leg_model_matrix, 
                        m4::y_rotation( deg_to_rad( -7. ).into() ) // 180 invert + 55 up
                    );
                }

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - MIDDLE_UPPER_LEG_WIDTH,
                        0.,
                        ( MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.,
                    )
                );

            }
            
            return upper_leg_model_matrix
        
        } else if leg_part == 1 {
            let mut joint_leg_model_matrix = m4::z_rotate_3_d(
                pre_matrix, 
                m4::z_rotation(deg_to_rad(-95.).into())
            );
            
            joint_leg_model_matrix = m4::translate_3_d( // INVERT THE NAMES
                joint_leg_model_matrix, 
                m4::translation(
                MIDDLE_JOINT_LEG_WIDTH * -1., // adjust distance by width 
                MIDDLE_UPPER_LEG_BIG_HEIGHT / 2. - MIDDLE_JOINT_LEG_BIG_HEIGHT / 2., // same y
                MIDDLE_UPPER_LEG_DEPTH / 2. - MIDDLE_JOINT_LEG_DEPTH / 2. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
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
    // in case of is upper leg dont forget to send the transformed pivot point
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


    // pub fn get_original_transformations(&self, pre_matrix: &[f32; 16], leg_i: usize) -> [[f32; 16]; 3] {
    //     let upper_leg_width: f32;
    //     let upper_leg_height: f32;
    //     let upper_leg_depth: f32;
    //     match self.position {
    //         LegType::Frontal => {
    //             // helps to start transformations at the right pivot point
    //             upper_leg_width = 0.;
    //             upper_leg_height = 0.;
    //             upper_leg_depth = 0.;
    //             // upper_leg_width = 0.;
    //             // upper_leg_height = FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.;
    //             // upper_leg_depth = FRONTAL_UPPER_LEG_DEPTH / 2.;
    //         },
    //         LegType::Back => {
    //             upper_leg_width = BACK_UPPER_LEG_WIDTH;
    //             upper_leg_height = 0.;
    //             upper_leg_depth = BACK_UPPER_LEG_DEPTH / 2.;
    //         },
    //         LegType::Middle => {
    //             upper_leg_width = MIDDLE_UPPER_LEG_WIDTH;
    //             upper_leg_height = MIDDLE_UPPER_LEG_SMALL_HEIGHT / 2.;
    //             upper_leg_depth = MIDDLE_UPPER_LEG_DEPTH;
    //         },
    //     }

    //     let pivot = ( 
    //         upper_leg_width, 
    //         upper_leg_height,
    //         upper_leg_depth 
    //     );

    //     let mut upper_leg_model_matrix = m4::translate_3_d( 
    //         *pre_matrix, // what happens with dereferencing a vector
    //         m4::translation( 
    //             self.body_clamp_point.0 - pivot.0, 
    //             self.body_clamp_point.1 - pivot.1, 
    //             self.body_clamp_point.2 - pivot.2, 
    //         )
    //     );
          
    //     let upper_leg_depth: f32;
    //     let joint_leg_depth: f32;
    //     let base_leg_depth: f32;
     
    //     let mut joint_leg_model_matrix: [f32; 16]; 
    //     let mut base_leg_model_matrix: [f32; 16];
    //     match self.position {
    //         LegType::Frontal => {
    //             let mut frontal_body_convergent_angle = get_frontal_body_convergent_angle() as f64;
    //             if leg_i == 1 {
    //                 frontal_body_convergent_angle *= -1.;
    //             }

    //             upper_leg_model_matrix = m4::y_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::y_rotation( frontal_body_convergent_angle )
    //             );

    //             upper_leg_model_matrix = m4::z_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::z_rotation( deg_to_rad( 55. ).into() ) 
    //             );

    //             upper_leg_model_matrix = m4::z_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::z_rotation( deg_to_rad( 180. ).into() ) 
    //             );

    //             upper_leg_model_matrix = m4::translate_3_d( // here is the pivot point
    //                 upper_leg_model_matrix, 
    //                 m4::translation( 
    //                     - FRONTAL_UPPER_LEG_WIDTH, 
    //                     FRONTAL_UPPER_LEG_BIG_HEIGHT / 2.,
    //                     - FRONTAL_UPPER_LEG_DEPTH / 2.
    //                 )
    //             );

    //             // joint leg transformations
    //             upper_leg_depth = FRONTAL_UPPER_LEG_DEPTH;
    //             joint_leg_depth = FRONTAL_JOINT_LEG_DEPTH;
    //             base_leg_depth = FRONTAL_BASE_LEG_DEPTH;
                
    //             joint_leg_model_matrix = m4::z_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::z_rotation(deg_to_rad(-102.).into())
    //             );
                
    //             joint_leg_model_matrix = m4::translate_3_d( // INVERT THE NAMES
    //                 joint_leg_model_matrix, 
    //                 m4::translation(
    //                 FRONTAL_JOINT_LEG_WIDTH * -1., // adjust distance by width 
    //                 FRONTAL_UPPER_LEG_BIG_HEIGHT / 2. - FRONTAL_JOINT_LEG_BIG_HEIGHT / 2., // same y
    //                 upper_leg_depth / 2. - joint_leg_depth / 2. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
    //                 )
    //             );
                
    //             // base leg transformations
    //             base_leg_model_matrix = m4::z_rotate_3_d(
    //                 joint_leg_model_matrix, 
    //                 m4::z_rotation( deg_to_rad(163.).into() )
    //             );    
    //             base_leg_model_matrix = m4::translate_3_d(
    //                 base_leg_model_matrix, 
    //                 m4::translation(
    //                 0.,
    //                 ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
    //                 (joint_leg_depth / 2.) - (base_leg_depth / 2.), 
    //                 )
    //             );

    //         },
    //         LegType::Back => {   
    //             upper_leg_model_matrix = m4::x_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::x_rotation( deg_to_rad( 180. ).into() )
    //             );

    //             upper_leg_model_matrix = m4::translate_3_d( 
    //                 upper_leg_model_matrix, // clamp matrix
    //                 m4::translation( 
    //                     FRONTAL_UPPER_LEG_WIDTH, 
    //                     0.,
    //                     - FRONTAL_UPPER_LEG_DEPTH
    //                 )
    //             );

    //             let mut back_body_convergent_angle = get_back_body_convergent_angle() as f64;
    //             if leg_i == 0 {
    //                 back_body_convergent_angle *= -1.;
    //             }

    //             upper_leg_model_matrix = m4::translate_3_d( 
    //                 upper_leg_model_matrix, // clamp matrix
    //                 m4::translation( 
    //                     0., 
    //                     - FRONTAL_UPPER_LEG_SMALL_HEIGHT,
    //                     FRONTAL_UPPER_LEG_DEPTH / 2.
    //                 )
    //             );

    //             upper_leg_model_matrix = m4::y_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::y_rotation( back_body_convergent_angle )
    //             );

    //             upper_leg_model_matrix = m4::translate_3_d( 
    //                 upper_leg_model_matrix, // clamp matrix
    //                 m4::translation( 
    //                     0., 
    //                     FRONTAL_UPPER_LEG_SMALL_HEIGHT,
    //                     - FRONTAL_UPPER_LEG_DEPTH / 2.
    //                 )
    //             );

    //             upper_leg_model_matrix = m4::z_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::z_rotation( deg_to_rad( 35. ).into() )
    //             );

    //             upper_leg_model_matrix = m4::translate_3_d( 
    //                 upper_leg_model_matrix, // clamp matrix
    //                 m4::translation( 
    //                     - FRONTAL_UPPER_LEG_WIDTH, 
    //                     0.,
    //                     0.
    //                 )
    //             );

    //             // joint leg transformations
    //             upper_leg_depth = BACK_UPPER_LEG_DEPTH;
    //             joint_leg_depth = BACK_JOINT_LEG_DEPTH;
    //             base_leg_depth = FRONTAL_BASE_LEG_DEPTH;  
                
    //             joint_leg_model_matrix = m4::z_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::z_rotation(deg_to_rad(-95.).into())
    //             );
                
    //             joint_leg_model_matrix = m4::translate_3_d( // INVERT THE NAMES
    //                 joint_leg_model_matrix, 
    //                 m4::translation(
    //                 FRONTAL_JOINT_LEG_WIDTH * -1., // adjust distance by width 
    //                 FRONTAL_UPPER_LEG_BIG_HEIGHT / 2. - FRONTAL_JOINT_LEG_BIG_HEIGHT / 2., // same y
    //                 upper_leg_depth / 2. - joint_leg_depth / 2. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
    //                 )
    //             );
                
    //             // base leg transformations
    //             base_leg_model_matrix = m4::z_rotate_3_d(
    //                 joint_leg_model_matrix, 
    //                 m4::z_rotation( deg_to_rad(163.).into() )
    //             );    
    //             base_leg_model_matrix = m4::translate_3_d(
    //                 base_leg_model_matrix, 
    //                 m4::translation(
    //                 0.,
    //                 ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
    //                 (joint_leg_depth / 2.) - (base_leg_depth / 2.), 
    //                 )
    //             );

                
    //         },
    //         LegType::Middle => {
    //             upper_leg_model_matrix = m4::translate_3_d( 
    //                 upper_leg_model_matrix, 
    //                 m4::translation( 
    //                     0.,
    //                     0.,
    //                     0.,
    //                 )
    //             );

    //             if leg_i < 3 {
    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         MIDDLE_UPPER_LEG_WIDTH,
    //                         (MIDDLE_UPPER_LEG_BIG_HEIGHT - MIDDLE_UPPER_LEG_SMALL_HEIGHT) * -1.,
    //                         MIDDLE_UPPER_LEG_DEPTH / 2.,
    //                     )
    //                 );

    //                 upper_leg_model_matrix = m4::y_rotate_3_d(
    //                     upper_leg_model_matrix, 
    //                     m4::y_rotation( deg_to_rad( -90. ).into() )
    //                 );
                   
    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         - MIDDLE_UPPER_LEG_WIDTH, 
    //                         MIDDLE_UPPER_LEG_BIG_HEIGHT - MIDDLE_UPPER_LEG_SMALL_HEIGHT,
    //                         ( MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.
    //                     )
    //                 );
    
    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         MIDDLE_UPPER_LEG_WIDTH,
    //                         0.,
    //                         MIDDLE_UPPER_LEG_DEPTH / 2.,
    //                     )
    //                 );
            
    //                 upper_leg_model_matrix = m4::x_rotate_3_d(
    //                     upper_leg_model_matrix, 
    //                     m4::x_rotation( deg_to_rad(-180.).into() ) // 180 invert + 55 up
    //                 );
    
    //                 upper_leg_model_matrix = m4::z_rotate_3_d(
    //                     upper_leg_model_matrix, 
    //                     m4::z_rotation( deg_to_rad(55.).into() ) // 180 invert + 55 up
    //                 );

    //                 // rotate the end legs in 15 degrees 
    //                 if leg_i == 0 {
    //                     upper_leg_model_matrix = m4::y_rotate_3_d(
    //                         upper_leg_model_matrix, 
    //                         m4::y_rotation( deg_to_rad( -7. ).into() ) // 180 invert + 55 up
    //                     );
    //                 }

    //                 if leg_i == 2 {
    //                     upper_leg_model_matrix = m4::y_rotate_3_d(
    //                         upper_leg_model_matrix, 
    //                         m4::y_rotation( deg_to_rad( 7. ).into() ) // 180 invert + 55 up
    //                     );
    //                 }
            
    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         - MIDDLE_UPPER_LEG_WIDTH,
    //                         0.,
    //                         (MIDDLE_UPPER_LEG_DEPTH / 2.) * -1.,
    //                     )
    //                 );
    //             } else {
    //                 // other side legs
    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         MIDDLE_UPPER_LEG_WIDTH,
    //                         0.,
    //                         MIDDLE_UPPER_LEG_DEPTH / 2.
    //                     )
    //                 );

    //                 upper_leg_model_matrix = m4::y_rotate_3_d(
    //                     upper_leg_model_matrix, 
    //                     m4::y_rotation( deg_to_rad( 90. ).into() )
    //                 );

    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         - MIDDLE_UPPER_LEG_WIDTH, 
    //                         0.,
    //                          (MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.,
    //                     )
    //                 );

    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         - MIDDLE_UPPER_LEG_WIDTH,
    //                         0.,
    //                         MIDDLE_UPPER_LEG_DEPTH / 2.,
    //                     )
    //                 );

    //                 upper_leg_model_matrix = m4::x_rotate_3_d(
    //                     upper_leg_model_matrix, 
    //                     m4::x_rotation( deg_to_rad( -180. ).into() ) // 180 invert + 55 up
    //                 );
                    
    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         MIDDLE_UPPER_LEG_WIDTH,
    //                         0.,
    //                         ( MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.,
    //                     )
    //                 );

    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         MIDDLE_UPPER_LEG_WIDTH,
    //                         0.,
    //                         MIDDLE_UPPER_LEG_DEPTH / 2.,
    //                     )
    //                 );

    //                 upper_leg_model_matrix = m4::z_rotate_3_d(
    //                     upper_leg_model_matrix, 
    //                     m4::z_rotation( deg_to_rad( 55.).into() ) 
    //                 );

    //                 // rotate the end legs in 15 degrees 
    //                 if leg_i == 3 {
    //                     upper_leg_model_matrix = m4::y_rotate_3_d(
    //                         upper_leg_model_matrix, 
    //                         m4::y_rotation( deg_to_rad( 7. ).into() ) // 180 invert + 55 up
    //                     );
    //                 }

    //                 if leg_i == 5 {
    //                     upper_leg_model_matrix = m4::y_rotate_3_d(
    //                         upper_leg_model_matrix, 
    //                         m4::y_rotation( deg_to_rad( -7. ).into() ) // 180 invert + 55 up
    //                     );
    //                 }

    //                 upper_leg_model_matrix = m4::translate_3_d( 
    //                     upper_leg_model_matrix, 
    //                     m4::translation( 
    //                         - MIDDLE_UPPER_LEG_WIDTH,
    //                         0.,
    //                         ( MIDDLE_UPPER_LEG_DEPTH / 2. ) * -1.,
    //                     )
    //                 );
    //             }

    //             // joint leg transformations
    //             upper_leg_depth = MIDDLE_UPPER_LEG_DEPTH;
    //             joint_leg_depth = MIDDLE_JOINT_LEG_DEPTH;
    //             base_leg_depth = MIDDLE_BASE_LEG_DEPTH; 
                
    //             joint_leg_model_matrix = m4::z_rotate_3_d(
    //                 upper_leg_model_matrix, 
    //                 m4::z_rotation(deg_to_rad(-95.).into())
    //             );
                
    //             joint_leg_model_matrix = m4::translate_3_d( // INVERT THE NAMES
    //                 joint_leg_model_matrix, 
    //                 m4::translation(
    //                 MIDDLE_JOINT_LEG_WIDTH * -1., // adjust distance by width 
    //                 MIDDLE_UPPER_LEG_BIG_HEIGHT / 2. - MIDDLE_JOINT_LEG_BIG_HEIGHT / 2., // same y
    //                 upper_leg_depth / 2. - joint_leg_depth / 2. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
    //                 )
    //             );
                
    //             // base leg transformations
    //             base_leg_model_matrix = m4::z_rotate_3_d(
    //                 joint_leg_model_matrix, 
    //                 m4::z_rotation( deg_to_rad(163.).into() )
    //             );    
    //             base_leg_model_matrix = m4::translate_3_d(
    //                 base_leg_model_matrix, 
    //                 m4::translation(
    //                 0.,
    //                 ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
    //                 (joint_leg_depth / 2.) - (base_leg_depth / 2.), 
    //                 )
    //             );

    //         }
    //     }
        
    
    //     [upper_leg_model_matrix, joint_leg_model_matrix, base_leg_model_matrix]
    // }

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
    pub middle_legs: [Leg; 6],
    control: SpiderControl
}

impl Spider {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let frontal_legs = [
            Leg::new(
                LegType::Frontal, 
                ( 
                    BODY_WIDTH - FRONTAL_LEG_INSET, 
                    BODY_HEIGHT / 2.15,
                    BODY_FRONTAL_DEPTH_OFFSET / 2.                
                )
            ),

            Leg::new(
                LegType::Frontal, 
                ( 
                    BODY_WIDTH - FRONTAL_LEG_INSET, 
                    BODY_HEIGHT / 2.15,
                    BODY_DEPTH - BODY_FRONTAL_DEPTH_OFFSET / 2.
                )   
            )
        ];

        let back_legs = [
            Leg::new(
                LegType::Back, 
                ( 
                    0., 
                    BODY_HEIGHT / 2.75,
                    BODY_DEPTH - BODY_BACK_DEPTH_OFFSET / 2.
                )
            ),

            Leg::new(
                LegType::Back, 
                ( 
                    0.,
                    BODY_HEIGHT / 2.75,
                    BODY_BACK_DEPTH_OFFSET / 2.
                )
            )
        ];

        let middle_legs = [
            Leg::new(
                LegType::Middle, 
                (  
                    BODY_WIDTH / 3. / 2.,
                    BODY_HEIGHT / 2.45,
                    FRONTAL_UPPER_LEG_DEPTH
                )
            ),
            Leg::new(
                LegType::Middle, 
                (  
                    (BODY_WIDTH / 3. * 2.) - BODY_WIDTH / 3. / 2.,
                    BODY_HEIGHT / 2.125,
                    FRONTAL_UPPER_LEG_DEPTH
                )
            ),
            Leg::new(
                LegType::Middle, 
                (  
                    (BODY_WIDTH / 3. * 3.) - BODY_WIDTH / 3. / 2.,
                    BODY_HEIGHT / 2.45,
                    FRONTAL_UPPER_LEG_DEPTH
                )
            ),

            // OTHER SIDE
            Leg::new(
                LegType::Middle, 
                (  
                    BODY_WIDTH / 3. / 2.,
                    BODY_HEIGHT / 2.45,
                    BODY_DEPTH,
                )
            ),
            Leg::new(
                LegType::Middle, 
                (  
                    (BODY_WIDTH / 3. * 2.) - BODY_WIDTH / 3. / 2.,
                    BODY_HEIGHT / 2.125,
                    BODY_DEPTH
                )
            ),
            Leg::new(
                LegType::Middle, 
                (  
                    (BODY_WIDTH / 3. * 3.) - BODY_WIDTH / 3. / 2.,
                    BODY_HEIGHT / 2.45,
                    BODY_DEPTH
                )
            ),
        ];

        Self { 
            control: SpiderControl::new(&canvas), // call it directly on the code
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
        }
    }

    pub fn animate_body(&mut self) {
        println!("animate body");
    }

    // fn change_direction(&mut self) {       
    //     match self.legs_direction {
    //         LegType::Back => self.legs_direction = LegType::Frontal,
    //         LegType::Frontal => self.legs_direction = LegType::Back,
    //         LegType::Middle => todo!(),
    //     }
    // }

    // pub fn animate_front_legs(&mut self, deltatime: f32) { // transfer into leg but re-understand logic before
    //     let mut displacement: f32 = self.speed * deltatime; // angle_displacement
                        
    //     if self.legs_direction == LegType::Back {
    //         displacement *= -1.;
    //     }
        
    //     self.z_acc_rotation += displacement;

    //     if !self.move_range.contains(&self.z_acc_rotation) {
    //         self.change_direction();
    //     }
    // }

    // pub fn animate_middle_legs(&mut self) {
    //     println!("animate middle legs");
    // }

    // pub fn animate_back_legs(&mut self) {
    //     println!("animate back legs");
    // }

    
}