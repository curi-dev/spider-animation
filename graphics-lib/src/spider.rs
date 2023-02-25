use std::ops::Range;

pub const FRONTAL_UPPER_LEG_SIZE: f32 = 0.;
pub const FRONTAL_MIDDLE_LEG_SIZE: f32 = 0.;
pub const FRONTAL_BOTTOM_LEG_SIZE: f32 = 0.;
pub const MIDDLE_UPPER_LEG_SIZE: f32 = 0.;
pub const MIDDLE_MIDDLE_LEG_SIZE: f32 = 0.;
pub const MIDDLE_BOTTOM_LEG_SIZE: f32 = 0.;
pub const BACK_UPPER_LEG_SIZE: f32 = 0.;
pub const BACK_MIDDLE_LEG_SIZE: f32 = 0.;
pub const BACK_BOTTOM_LEG_SIZE: f32 = 0.;

enum LegPosition {
    Frontal,
    Back,
    Middle
}
struct Leg {
    position: LegPosition,
    upper_leg_move_range: Range<f32>,
    middle_leg_move_range: Range<f32>,
    bottom_leg_move_range: Range<f32>,
    upper_leg_data: [f32; 108],
    middle_leg_data: [f32; 108], 
    bottom_leg_data: [f32; 90],
}

impl Leg {
    fn new(position: LegPosition) -> Self {
        let apex: (f32, f32, f32) = (26., 5., 1.5); 

        // use the constants to determine the size
        let upper_leg_data: [f32; 108];
        let middle_leg_data: [f32; 108];
        let bottom_leg_data: [f32; 90];
        match position {
            LegPosition::Frontal => {
                upper_leg_data = [
                    // front side face          
                    0., 0., 0.,
                    0., 12., 0.,
                    26., 1., 0.,
            
                    26., 1., 0.,
                    0., 12., 0.,
                    26., 11., 0.,
        
                    // back side face
                    0., 0., 3.,
                    26., 1., 3., //
                    0., 12., 3.,
            
                    26., 1., 3.,
                    26., 11., 3.,
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    26., 1., 0.,
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    26., 1., 0.,
                    26., 1., 3.,
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    26., 11., 0.,
        
                    0., 12., 3.,
                    26., 11., 3.,
                    26., 11., 0.,
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
        
                    // between front and back side II
                    26., 1., 0.,
                    26., 11., 0.,
                    26., 1., 3., 
                    
                    26., 1., 3., 
                    26., 11., 0.,
                    26., 11., 3.
                ];

                middle_leg_data =  [
                    // front side face          
                    0., 0., 0.,
                    0., 12., 0.,
                    26., 1., 0.,
            
                    26., 1., 0.,
                    0., 12., 0.,
                    26., 11., 0.,
        
                    // back side face
                    0., 0., 3.,
                    26., 1., 3., //
                    0., 12., 3.,
            
                    26., 1., 3.,
                    26., 11., 3.,
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    26., 1., 0.,
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    26., 1., 0.,
                    26., 1., 3.,
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    26., 11., 0.,
        
                    0., 12., 3.,
                    26., 11., 3.,
                    26., 11., 0.,
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
        
                    // between front and back side II
                    26., 1., 0.,
                    26., 11., 0.,
                    26., 1., 3., 
                    
                    26., 1., 3., 
                    26., 11., 0.,
                    26., 11., 3.
                ];

                bottom_leg_data = [
                    0., 0., 0.,
                    0., 12., 0.,
                    apex.0, apex.1, apex.2, 
            
                    apex.0, apex.1, apex.2, 
                    0., 12., 0.,
                    apex.0, apex.1, apex.2, 
        
                    // back side face
                    0., 0., 3.,
                    apex.0, apex.1, apex.2, 
                    0., 12., 3.,
            
                    26., 1., 3.,
                    apex.0, apex.1, apex.2, 
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    apex.0, apex.1, apex.2, 
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    apex.0, apex.1, apex.2, 
                    apex.0, apex.1, apex.2, 
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    apex.0, apex.1, apex.2, 
        
                    0., 12., 3.,
                    apex.0, apex.1, apex.2, 
                    apex.0, apex.1, apex.2, 
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
                ]
            },
            LegPosition::Back => {
                upper_leg_data = [
                    // front side face          
                    0., 0., 0.,
                    0., 12., 0.,
                    26., 1., 0.,
            
                    26., 1., 0.,
                    0., 12., 0.,
                    26., 11., 0.,
        
                    // back side face
                    0., 0., 3.,
                    26., 1., 3., //
                    0., 12., 3.,
            
                    26., 1., 3.,
                    26., 11., 3.,
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    26., 1., 0.,
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    26., 1., 0.,
                    26., 1., 3.,
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    26., 11., 0.,
        
                    0., 12., 3.,
                    26., 11., 3.,
                    26., 11., 0.,
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
        
                    // between front and back side II
                    26., 1., 0.,
                    26., 11., 0.,
                    26., 1., 3., 
                    
                    26., 1., 3., 
                    26., 11., 0.,
                    26., 11., 3.
                ];

                middle_leg_data =  [
                    // front side face          
                    0., 0., 0.,
                    0., 12., 0.,
                    26., 1., 0.,
            
                    26., 1., 0.,
                    0., 12., 0.,
                    26., 11., 0.,
        
                    // back side face
                    0., 0., 3.,
                    26., 1., 3., //
                    0., 12., 3.,
            
                    26., 1., 3.,
                    26., 11., 3.,
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    26., 1., 0.,
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    26., 1., 0.,
                    26., 1., 3.,
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    26., 11., 0.,
        
                    0., 12., 3.,
                    26., 11., 3.,
                    26., 11., 0.,
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
        
                    // between front and back side II
                    26., 1., 0.,
                    26., 11., 0.,
                    26., 1., 3., 
                    
                    26., 1., 3., 
                    26., 11., 0.,
                    26., 11., 3.
                ];

                bottom_leg_data = [
                    0., 0., 0.,
                    0., 12., 0.,
                    apex.0, apex.1, apex.2, 
            
                    apex.0, apex.1, apex.2, 
                    0., 12., 0.,
                    apex.0, apex.1, apex.2, 
        
                    // back side face
                    0., 0., 3.,
                    apex.0, apex.1, apex.2, 
                    0., 12., 3.,
            
                    26., 1., 3.,
                    apex.0, apex.1, apex.2, 
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    apex.0, apex.1, apex.2, 
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    apex.0, apex.1, apex.2, 
                    apex.0, apex.1, apex.2, 
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    apex.0, apex.1, apex.2, 
        
                    0., 12., 3.,
                    apex.0, apex.1, apex.2, 
                    apex.0, apex.1, apex.2, 
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
                ]
            },
            LegPosition::Middle => {
                upper_leg_data = [
                    // front side face          
                    0., 0., 0.,
                    0., 12., 0.,
                    26., 1., 0.,
            
                    26., 1., 0.,
                    0., 12., 0.,
                    26., 11., 0.,
        
                    // back side face
                    0., 0., 3.,
                    26., 1., 3., //
                    0., 12., 3.,
            
                    26., 1., 3.,
                    26., 11., 3.,
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    26., 1., 0.,
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    26., 1., 0.,
                    26., 1., 3.,
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    26., 11., 0.,
        
                    0., 12., 3.,
                    26., 11., 3.,
                    26., 11., 0.,
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
        
                    // between front and back side II
                    26., 1., 0.,
                    26., 11., 0.,
                    26., 1., 3., 
                    
                    26., 1., 3., 
                    26., 11., 0.,
                    26., 11., 3.
                ];

                middle_leg_data =  [
                    // front side face          
                    0., 0., 0.,
                    0., 12., 0.,
                    26., 1., 0.,
            
                    26., 1., 0.,
                    0., 12., 0.,
                    26., 11., 0.,
        
                    // back side face
                    0., 0., 3.,
                    26., 1., 3., //
                    0., 12., 3.,
            
                    26., 1., 3.,
                    26., 11., 3.,
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    26., 1., 0.,
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    26., 1., 0.,
                    26., 1., 3.,
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    26., 11., 0.,
        
                    0., 12., 3.,
                    26., 11., 3.,
                    26., 11., 0.,
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
        
                    // between front and back side II
                    26., 1., 0.,
                    26., 11., 0.,
                    26., 1., 3., 
                    
                    26., 1., 3., 
                    26., 11., 0.,
                    26., 11., 3.
                ];
                
                bottom_leg_data = [
                    0., 0., 0.,
                    0., 12., 0.,
                    apex.0, apex.1, apex.2, 
            
                    apex.0, apex.1, apex.2, 
                    0., 12., 0.,
                    apex.0, apex.1, apex.2, 
        
                    // back side face
                    0., 0., 3.,
                    apex.0, apex.1, apex.2, 
                    0., 12., 3.,
            
                    26., 1., 3.,
                    apex.0, apex.1, apex.2, 
                    0., 12., 3.,
        
                    // upper face
                    0., 0., 0.,
                    apex.0, apex.1, apex.2, 
                    0., 0., 3.,
                    
                    0., 0., 3.,
                    apex.0, apex.1, apex.2, 
                    apex.0, apex.1, apex.2, 
        
                    // bottom face
                    0., 12., 0.,
                    0., 12., 3.,
                    apex.0, apex.1, apex.2, 
        
                    0., 12., 3.,
                    apex.0, apex.1, apex.2, 
                    apex.0, apex.1, apex.2, 
        
                    // between front and back side I
                    0., 0., 0.,
                    0., 0., 3.,
                    0., 12., 0.,
        
                    0., 12., 0.,
                    0., 0., 3.,
                    0., 12., 3.,
                ]
            },
        }

        Self {
            position,
            upper_leg_data,
            middle_leg_data,
            bottom_leg_data,
            upper_leg_move_range: Range { start: 0., end: 11. },
            middle_leg_move_range: Range { start: 0., end: 11. },
            bottom_leg_move_range: Range { start: 0., end: 11. },
        }
    }
}

pub struct Spider {
    pub legs_direction: LegsDirection,
    pub move_range: Range<f32>,
    pub upper_and_middle_legs_data: [f32; 108],
    pub bottom_legs_data: [f32; 90],
    pub colors: [u8; 108],
    pub speed: f32,
    pub x_acc_rotation: f32,
    pub y_acc_rotation: f32,
    pub z_acc_rotation: f32, // for front legs
    pub initial_displacement_x: f32,
    pub initial_displacement_y: f32,
    pub initial_displacement_z: f32,
    pub pivot_point: (f32, f32, f32),
    legs: [Leg; 1]
}

impl Spider {
    pub fn new() -> Self {
        let legs = [
            Leg::new(LegPosition::Frontal)
        ];

        let initial_displacement_x = 300. as f32; 
        let initial_displacement_y = 200. as f32; 
        let initial_displacement_z = 0. as f32; 
        let apex: (f32, f32, f32) = (26., 5., 1.5); 

        Self { 
            legs,
            pivot_point: apex,
            initial_displacement_x,
            initial_displacement_y,
            initial_displacement_z,
            legs_direction: LegsDirection::Front,
            speed: 10., 
            move_range:  Range { start: 0., end: 11. },
            z_acc_rotation: 0.,
            x_acc_rotation: 0.,
            y_acc_rotation: 0.,
            upper_and_middle_legs_data: [
                // front side face          
                0., 0., 0.,
                0., 12., 0.,
                26., 1., 0.,
        
                26., 1., 0.,
                0., 12., 0.,
                26., 11., 0.,
    
                // back side face
                0., 0., 3.,
                26., 1., 3., //
                0., 12., 3.,
        
                26., 1., 3.,
                26., 11., 3.,
                0., 12., 3.,
    
                // upper face
                0., 0., 0.,
                26., 1., 0.,
                0., 0., 3.,
                
                0., 0., 3.,
                26., 1., 0.,
                26., 1., 3.,
    
                // bottom face
                0., 12., 0.,
                0., 12., 3.,
                26., 11., 0.,
    
                0., 12., 3.,
                26., 11., 3.,
                26., 11., 0.,
    
                // between front and back side I
                0., 0., 0.,
                0., 0., 3.,
                0., 12., 0.,
    
                0., 12., 0.,
                0., 0., 3.,
                0., 12., 3.,
    
                // between front and back side II
                26., 1., 0.,
                26., 11., 0.,
                26., 1., 3., 
                
                26., 1., 3., 
                26., 11., 0.,
                26., 11., 3.
            ],
            bottom_legs_data: [
                0., 0., 0.,
                0., 12., 0.,
                apex.0, apex.1, apex.2, 
        
                apex.0, apex.1, apex.2, 
                0., 12., 0.,
                apex.0, apex.1, apex.2, 
    
                // back side face
                0., 0., 3.,
                apex.0, apex.1, apex.2, 
                0., 12., 3.,
        
                26., 1., 3.,
                apex.0, apex.1, apex.2, 
                0., 12., 3.,
    
                // upper face
                0., 0., 0.,
                apex.0, apex.1, apex.2, 
                0., 0., 3.,
                
                0., 0., 3.,
                apex.0, apex.1, apex.2, 
                apex.0, apex.1, apex.2, 
    
                // bottom face
                0., 12., 0.,
                0., 12., 3.,
                apex.0, apex.1, apex.2, 
    
                0., 12., 3.,
                apex.0, apex.1, apex.2, 
                apex.0, apex.1, apex.2, 
    
                // between front and back side I
                0., 0., 0.,
                0., 0., 3.,
                0., 12., 0.,
    
                0., 12., 0.,
                0., 0., 3.,
                0., 12., 3.,
            ],
            colors: [
                // front side face - rose
                200,  70, 120,
                200,  70, 120,
                200,  70, 120,
    
                200,  70, 120,
                200,  70, 120,
                200,  70, 120,
     
                // back side face - purple
                80, 70, 200,
                80, 70, 200,
                80, 70, 200,
    
                80, 70, 200,
                80, 70, 200,
                80, 70, 200,
    
                // upper face
                90, 130, 110,
                90, 130, 110,
                90, 130, 110,
    
                90, 130, 110,
                90, 130, 110,
                90, 130, 110,
    
                // bottom face
                160, 160, 220,
                160, 160, 220,
                160, 160, 220,
                160, 160, 220,
                160, 160, 220,
                160, 160, 220,
    
                // between front and back side I
                76, 210, 100,
                76, 210, 100,
                76, 210, 100,
                76, 210, 100,
                76, 210, 100,
                76, 210, 100,
    
                // between front and back side II
                95, 215, 150,
                95, 215, 150,
                95, 215, 150,
                95, 215, 150,
                95, 215, 150,
                95, 215, 150,
            ],
        }
    }

    fn change_direction(&mut self) {       
        match self.legs_direction {
            LegsDirection::Back => self.legs_direction = LegsDirection::Front,
            LegsDirection::Front => self.legs_direction = LegsDirection::Back,
        }
    }

    pub fn animate_front_legs(&mut self, deltatime: f32) {     
        let mut displacement: f32 = self.speed * deltatime; // angle_displacement
                        
        if self.legs_direction == LegsDirection::Back {
            displacement *= -1.;
        }
        
        self.z_acc_rotation += displacement;

        if !self.move_range.contains(&self.z_acc_rotation) {
            self.change_direction();
        }
    }

    pub fn animate_middle_legs(&mut self) {
        println!("animate middle legs");
    }

    pub fn animate_back_legs(&mut self) {
        println!("animate back legs");
    }

    pub fn animate_body(&mut self) {
        println!("animate body");
    }

    pub fn get_curr_pivot_point_of_upper_leg(&self) -> (f32, f32, f32) { // consider current translation (i can use accumulate displacement on move)
        (
            self.initial_displacement_x + self.pivot_point.0, 
            self.initial_displacement_y + self.pivot_point.1, 
            self.initial_displacement_z + self.pivot_point.2
        )    
    }
}

#[derive(PartialEq)]
pub enum LegsDirection {
    Back,
    Front
}