use std::ops::Range;



pub struct Spider {
    pub legs_direction: LegsDirection,
    pub move_range: Range<f32>,
    pub upper_and_middle_legs_data: [f32; 108],
    pub bottom_legs_data: [f32; 90],
    pub colors: [u8; 108]
}

impl Spider {
    pub fn new() -> Self {
        let apex: (f32, f32, f32) = (26., 5., 1.5); 

        Self { 
            legs_direction: LegsDirection::Front, 
            move_range:  Range { start: 0., end: 11. },
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
                26., 1., 3.,
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
            ]
        
        }
    }
    pub fn change_direction(&mut self) {
        
        match self.legs_direction {
            LegsDirection::Back => self.legs_direction = LegsDirection::Front,
            LegsDirection::Front => self.legs_direction = LegsDirection::Back,
        }
    }

    pub fn animate_front_legs(&mut self) {

    }

    pub fn animate_middle_legs(&mut self) {

    }

    pub fn animate_back_legs(&mut self) {

    }
}

#[derive(PartialEq)]
pub enum LegsDirection {
    Back,
    Front
}