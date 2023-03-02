use std::ops::Range;
use crate::data_structures::{get_colors, get_leg_data, get_base_leg_colors, get_body_colors, get_body_data};

#[derive(PartialEq)]
pub enum LegPosition {
    Frontal,
    Back,
    Middle
}

pub struct Leg {
    pub position: LegPosition,
    pub upper_leg_move_range: Range<f32>,
    pub middle_leg_move_range: Range<f32>,
    pub bottom_leg_move_range: Range<f32>,
    pub upper_leg_data: [f32; 108],
    pub middle_leg_data: [f32; 108], 
    //pub bottom_leg_data: [f32; 90],
    pub bottom_leg_data: [f32; 54],
    pub z_acc_rotation: f32,
}

impl Leg {
    fn new(leg_position: LegPosition) -> Self {
        // use the constants to determine the size
        let leg_data = get_leg_data(&leg_position);
    
        Self {
            position: leg_position,
            upper_leg_data: leg_data.0,
            middle_leg_data: leg_data.1,
            bottom_leg_data: leg_data.2,
            upper_leg_move_range: Range { start: 0., end: 11. },
            middle_leg_move_range: Range { start: 0., end: 11. },
            bottom_leg_move_range: Range { start: 0., end: 11. },
            z_acc_rotation: 0.
        }
    }
}

// separate legs and body into a structure itself
pub struct Spider {
    pub legs_direction: LegPosition,
    pub move_range: Range<f32>,
    pub colors: [u8; 108],
    pub base_leg_colors: [u8; 54],
    pub body_colors: [u8; 270],
    pub body_data: [f32; 270], // separate body
    pub speed: f32,
    pub x_acc_rotation: f32,
    pub y_acc_rotation: f32,
    pub z_acc_rotation: f32, // for front legs
    pub frontal_legs: [Leg; 1]
}

impl Spider {
    pub fn new() -> Self {
        let frontal_legs = [
            Leg::new(LegPosition::Frontal)
        ];

        Self { 
            frontal_legs,        
            legs_direction: LegPosition::Frontal,
            speed: 10., 
            move_range:  Range { start: 0., end: 11. },
            z_acc_rotation: 0.,
            x_acc_rotation: 0.,
            y_acc_rotation: 0.,
            colors: get_colors(),
            base_leg_colors: get_base_leg_colors(),
            body_colors: get_body_colors(),
            body_data: get_body_data()
        }
    }

    fn change_direction(&mut self) {       
        match self.legs_direction {
            LegPosition::Back => self.legs_direction = LegPosition::Frontal,
            LegPosition::Frontal => self.legs_direction = LegPosition::Back,
            LegPosition::Middle => todo!(),
        }
    }

    pub fn animate_front_legs(&mut self, deltatime: f32) { // transfer into leg but re-understand logic before
        let mut displacement: f32 = self.speed * deltatime; // angle_displacement
                        
        if self.legs_direction == LegPosition::Back {
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
}


