use std::ops::Range;
use crate::{
    data_structures::{
        get_colors, 
        get_leg_data, 
        get_base_leg_colors, 
        get_body_colors, 
        get_body_data, get_head_data
    }, 
    modules::m4::m4::M4 as m4, 
    webgl_utils::deg_to_rad, 
    constants::*, log
};

#[derive(PartialEq)]
pub enum LegType {
    Frontal,
    Back,
    Middle
}

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
    fn new(leg_position: LegType, body_clamp_point: (f32, f32, f32)) -> Self {
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

    pub fn animate(&mut self, deltatime: f32) { // transfer into leg but re-understand logic before
        println!("animate this self");
        // let mut displacement: f32 = self.speed * deltatime; 
                        
        // if self.legs_direction == LegType::Back {
        //     displacement *= -1.;
        // }
        
        // self.z_acc_rotation += displacement;

        // if !self.move_range.contains(&self.z_acc_rotation) {
        //     self.change_direction();
        // }
    }

    fn change_direction(&mut self) {       
        println!("change direction");
    }

    pub fn set_pivot_point(&self, pre_matrix: &[f32; 16]) -> [f32; 16] {
        let upper_leg_width: f32;
        let upper_leg_height: f32;
        let upper_leg_depth: f32;
        match self.position {
            LegType::Frontal => {
                upper_leg_width = FRONTAL_UPPER_LEG_WIDTH;
                upper_leg_height = 0.;
                upper_leg_depth = FRONTAL_UPPER_LEG_DEPTH / 2.;
            },
            LegType::Back => {
                upper_leg_width = BACK_UPPER_LEG_WIDTH;
                upper_leg_height = 0.;
                upper_leg_depth = BACK_UPPER_LEG_DEPTH / 2.;
            },
            LegType::Middle => {
                upper_leg_width = MIDDLE_UPPER_LEG_WIDTH;
                upper_leg_height = MIDDLE_UPPER_LEG_SMALL_HEIGHT / 2.;
                upper_leg_depth = MIDDLE_UPPER_LEG_DEPTH;
            },
        }

        let pivot = ( // CHANGE THIS NAME TO SELF_PIVOT_POINT
            upper_leg_width, 
            upper_leg_height,
            upper_leg_depth 
        );

        let upper_leg_model_matrix = m4::translate_3_d( 
            *pre_matrix, // what happens with dereferencing a vector
            m4::translation( 
                self.body_clamp_point.0 - pivot.0, 
                self.body_clamp_point.1 - pivot.1, 
                self.body_clamp_point.2 - pivot.2, 
            )
        );
        
        upper_leg_model_matrix
    }


    pub fn set_initial_transformations(&self, pre_matrix: &[f32; 16], leg_i: usize) -> [[f32; 16]; 3] {
        let upper_leg_depth: f32;
        let joint_leg_depth: f32;
        let base_leg_depth: f32;
        let mut upper_leg_model_matrix: [f32; 16]; 
        let mut joint_leg_model_matrix: [f32; 16]; 
        let mut base_leg_model_matrix: [f32; 16];
        match self.position {
            LegType::Frontal => {
                upper_leg_model_matrix = m4::translate_3_d( 
                    *pre_matrix, 
                    m4::translation( 
                        FRONTAL_UPPER_LEG_WIDTH, 
                        BODY_HEIGHT * 0.10,
                        0.
                    )
                );
        
                upper_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad( 180. ).into() ) // 180 invert + 55 up
                );

        
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - FRONTAL_UPPER_LEG_WIDTH, 
                        0.,
                        0.
                    )
                );

                // LAST ROTATION
                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        FRONTAL_UPPER_LEG_WIDTH, 
                        FRONTAL_UPPER_LEG_SMALL_HEIGHT,
                        FRONTAL_UPPER_LEG_DEPTH / 2.
                    )
                );

                let mut frontal_body_convergent_angle = (get_frontal_body_convergent_angle() as f64);
                if leg_i == 0 {
                    frontal_body_convergent_angle *= -1.;
                }
        
                log(&format!("frontal body convergent angle: {:?} ", frontal_body_convergent_angle));
        
                upper_leg_model_matrix = m4::y_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::y_rotation( frontal_body_convergent_angle )
                );

                upper_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad( 55. ).into() ) // 180 invert + 55 up
                );

                upper_leg_model_matrix = m4::translate_3_d( 
                    upper_leg_model_matrix, 
                    m4::translation( 
                        - FRONTAL_UPPER_LEG_WIDTH, 
                        - FRONTAL_UPPER_LEG_SMALL_HEIGHT,
                        - FRONTAL_UPPER_LEG_DEPTH / 2.
                    )
                );

                // joint leg transformations
                upper_leg_depth = FRONTAL_UPPER_LEG_DEPTH;
                joint_leg_depth = FRONTAL_JOINT_LEG_DEPTH;
                base_leg_depth = FRONTAL_BASE_LEG_DEPTH;
                
                joint_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation(deg_to_rad(-102.).into())
                );
                
                joint_leg_model_matrix = m4::translate_3_d( // INVERT THE NAMES
                    joint_leg_model_matrix, 
                    m4::translation(
                    FRONTAL_JOINT_LEG_WIDTH * -1., // adjust distance by width 
                    FRONTAL_UPPER_LEG_BIG_HEIGHT / 2. - FRONTAL_JOINT_LEG_BIG_HEIGHT / 2., // same y
                    upper_leg_depth / 2. - joint_leg_depth / 2. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
                    )
                );
                
                // base leg transformations
                base_leg_model_matrix = m4::z_rotate_3_d(
                    joint_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad(163.).into() )
                );    
                base_leg_model_matrix = m4::translate_3_d(
                    base_leg_model_matrix, 
                    m4::translation(
                    0.,
                    ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
                    (joint_leg_depth / 2.) - (base_leg_depth / 2.), 
                    )
                );

            },
            LegType::Back => {   
                upper_leg_model_matrix = m4::x_rotate_3_d(
                    *pre_matrix, 
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

                // joint leg transformations
                upper_leg_depth = BACK_UPPER_LEG_DEPTH;
                joint_leg_depth = BACK_JOINT_LEG_DEPTH;
                base_leg_depth = FRONTAL_BASE_LEG_DEPTH;  
                
                joint_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation(deg_to_rad(-95.).into())
                );
                
                joint_leg_model_matrix = m4::translate_3_d( // INVERT THE NAMES
                    joint_leg_model_matrix, 
                    m4::translation(
                    FRONTAL_JOINT_LEG_WIDTH * -1., // adjust distance by width 
                    FRONTAL_UPPER_LEG_BIG_HEIGHT / 2. - FRONTAL_JOINT_LEG_BIG_HEIGHT / 2., // same y
                    upper_leg_depth / 2. - joint_leg_depth / 2. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
                    )
                );
                
                // base leg transformations
                base_leg_model_matrix = m4::z_rotate_3_d(
                    joint_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad(163.).into() )
                );    
                base_leg_model_matrix = m4::translate_3_d(
                    base_leg_model_matrix, 
                    m4::translation(
                    0.,
                    ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
                    (joint_leg_depth / 2.) - (base_leg_depth / 2.), 
                    )
                );

                
            },
            LegType::Middle => {
                upper_leg_model_matrix = m4::translate_3_d( 
                    *pre_matrix, 
                    m4::translation( 
                        0.,
                        0.,
                        0.,
                    )
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

                // joint leg transformations
                upper_leg_depth = MIDDLE_UPPER_LEG_DEPTH;
                joint_leg_depth = MIDDLE_JOINT_LEG_DEPTH;
                base_leg_depth = MIDDLE_BASE_LEG_DEPTH; 
                
                joint_leg_model_matrix = m4::z_rotate_3_d(
                    upper_leg_model_matrix, 
                    m4::z_rotation(deg_to_rad(-95.).into())
                );
                
                joint_leg_model_matrix = m4::translate_3_d( // INVERT THE NAMES
                    joint_leg_model_matrix, 
                    m4::translation(
                    MIDDLE_JOINT_LEG_WIDTH * -1., // adjust distance by width 
                    MIDDLE_UPPER_LEG_BIG_HEIGHT / 2. - MIDDLE_JOINT_LEG_BIG_HEIGHT / 2., // same y
                    upper_leg_depth / 2. - joint_leg_depth / 2. // adjust depth (1.5) - do not need to have same depth because only upper is rotating by its center
                    )
                );
                
                // base leg transformations
                base_leg_model_matrix = m4::z_rotate_3_d(
                    joint_leg_model_matrix, 
                    m4::z_rotation( deg_to_rad(163.).into() )
                );    
                base_leg_model_matrix = m4::translate_3_d(
                    base_leg_model_matrix, 
                    m4::translation(
                    0.,
                    ((FRONTAL_JOINT_LEG_BIG_HEIGHT / 2.) + (FRONTAL_BOTTOM_LEG_HEIGHT / 2.)) * -1.,
                    (joint_leg_depth / 2.) - (base_leg_depth / 2.), 
                    )
                );

            }
        }
        
    
        [upper_leg_model_matrix, joint_leg_model_matrix, base_leg_model_matrix]
    }
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
}

impl Spider {
    pub fn new() -> Self {
        let frontal_legs = [
            Leg::new(
                LegType::Frontal, 
                ( 
                    BODY_WIDTH, 
                    BODY_HEIGHT / 2.75,
                    BODY_FRONTAL_DEPTH_OFFSET / 1.5
                )
            ),

            Leg::new(
                LegType::Frontal, 
                ( 
                    BODY_WIDTH, 
                    BODY_HEIGHT / 2.75,
                    BODY_DEPTH - BODY_FRONTAL_DEPTH_OFFSET / 1.5
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
            base_leg_colors: get_base_leg_colors(), // call it directly on the code
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


