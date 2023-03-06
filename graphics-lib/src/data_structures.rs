use crate::spider::LegType;
use crate::constants::*;

fn get_upper_or_middle_leg_data(width: f32, big_height: f32, small_height: f32, depth: f32) -> Vec<f32> {
    let upper_leg_data = vec![
        // front side face          
        0., 0., 0.,
        0., big_height, 0.,
        width, (big_height - small_height) / 2., 0.,
    
        width, (big_height - small_height) / 2., 0.,
        0., big_height, 0.,
        width, small_height, 0.,
    
        // back side face
        0., 0., depth,
        width, (big_height - small_height) / 2., depth, //
        0., big_height, depth,
    
        width, (big_height - small_height) / 2., depth,
        width, small_height, depth,
        0., big_height, depth,
    
        // upper face
        0., 0., 0.,
        width, (big_height - small_height) / 2., 0.,
        0., 0., depth,
        
        0., 0., depth,
        width, (big_height - small_height) / 2., 0.,
        width, (big_height - small_height) / 2., depth,
    
        // bottom face
        0., big_height, 0.,
        0., big_height, depth,
        width, small_height, 0.,
    
        0., big_height, depth,
        width, small_height, depth,
        width, small_height, 0.,
    
        // between front and back side I
        0., 0., 0.,
        0., 0., depth,
        0., big_height, 0.,
    
        0., big_height, 0.,
        0., 0., depth,
        0., big_height, depth,
    
        // between front and back side II
        width, (big_height - small_height) / 2., 0.,
        width, small_height, 0.,
        width, (big_height - small_height) / 2., depth, 
        
        width, (big_height - small_height) / 2., depth, 
        width, small_height, 0.,
        width, small_height, depth
    ];
    
    upper_leg_data
}

fn get_bottom_leg_data(width: f32, height: f32, depth: f32) -> Vec<f32> {
    let apex = (width, height / 2., depth / 2.);
    
    let bottom_leg_data =vec![
        // front side face
        0., 0., 0.,
        0., height, 0.,
        apex.0, apex.1, apex.2, 

        // back side face
        0., 0., depth,
        apex.0, apex.1, apex.2, 
        0., height, depth,

        // upper face
        0., 0., 0.,
        apex.0, apex.1, apex.2, 
        0., 0., depth,
        
        // // bottom face
        0., height, 0.,
        0., height, depth,
        apex.0, apex.1, apex.2, 

        // // between front and back side I [REVIEW]
        0., 0., 0.,
        0., 0., depth,
        0., height, 0.,

        0., height, 0.,
        0., 0., depth,
        0., height, depth,
    ];

    bottom_leg_data
}

pub fn get_base_leg_colors() -> [u8; 54] {
    let colors = [
        // front side face - rose
        200,  70, 120,
        200,  70, 120,
        200,  70, 120,

        // back side face - purple
        80, 70, 200,
        80, 70, 200,
        80, 70, 200,

        // upper face
        90, 130, 110,
        90, 130, 110,
        90, 130, 110,

        // bottom face
        160, 160, 220,
        160, 160, 220,
        160, 160, 220,

        // between front and back side I [REVIEW]
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
    ];

    colors
}

pub fn get_colors() -> [u8; 108] {

    let colors = [
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
    ];

    colors
}

pub fn get_body_colors() -> [u8; 270] {
    let colors = [
        // side one
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
 
        // side two
        95, 215, 150,
        95, 215, 150,
        95, 215, 150,
        95, 215, 150,
        95, 215, 150,
        95, 215, 150,

        // convergent 1 to between A
        60, 60, 220,
        60, 60, 220,
        60, 60, 220,
        60, 60, 220,
        60, 60, 220,
        60, 60, 220,

        // convergent 2 to between A
        110, 160, 180,
        110, 160, 180,
        110, 160, 180,
        110, 160, 180,
        110, 160, 180,
        110, 160, 180,

        // between A
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,

        // upside
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,

        // downside
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,

        // triangle one up side close
        0, 0, 100,
        0, 0, 100,
        0, 0, 100,

        // triangle two up side close
        200, 0, 100,
        200, 0, 100,
        200, 0, 100,

        // close between one
        30, 80, 44,
        30, 80, 44,
        30, 80, 44,

        // CLOSE BETWEEN DOWN 

        // close between one
        70, 5, 144,
        70, 5, 144,
        70, 5, 144,

        // triangle one up side close
        0, 0, 100,
        0, 0, 100,
        0, 0, 100,

        // triangle two up side close
        200, 0, 100,
        200, 0, 100,
        200, 0, 100,

        // close between one & two
        30, 80, 44,
        30, 80, 44,
        30, 80, 44,
        70, 5, 144,
        70, 5, 144,
        70, 5, 144,

        // tail convergent one
        200, 15, 44,
        200, 15, 44,
        200, 15, 44,
        200, 15, 44,
        200, 15, 44,
        200, 15, 44,

        // tail convergent two
        216, 1, 24,
        216, 1, 24,
        216, 1, 24,
        216, 1, 24,
        216, 1, 24,
        216, 1, 24,

        // tail close triangle up
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
 
        // tail close triangle down
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
        76, 210, 100,
    ];

    colors
}

pub fn get_body_data() -> [f32; 270] {
    let body_data = [
        // side one
        0., 0., 0.,
        0., BODY_HEIGHT, 0., // height
        BODY_WIDTH, 0., 0., // width
        
        BODY_WIDTH, 0., 0., // width
        0., BODY_HEIGHT, 0., // height
        BODY_WIDTH, BODY_HEIGHT, 0., // width, height

        // side two
        0., 0., BODY_DEPTH, // depth
        BODY_WIDTH, 0., BODY_DEPTH, // width, depth
        0., BODY_HEIGHT, BODY_DEPTH, // height, depth
        
        BODY_WIDTH, 0., BODY_DEPTH, // width, depth
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH, // width, height, depth
        0., BODY_HEIGHT, BODY_DEPTH, // height, depth

        // convergent 1 to between A
        BODY_WIDTH, 0., 0., // width
        BODY_WIDTH, BODY_HEIGHT, 0., // width, height
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH / 3., // width advance convergent (width + 10), height, depth / 2
        
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH / 3., // width advance convergent, height, depth / 2
        BODY_WIDTH, BODY_HEIGHT, 0.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 3.,

        // convergent 2 to between A
        BODY_WIDTH, 0., BODY_DEPTH, // width
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH - (BODY_DEPTH / 3.), // width advance convergent (width + 10), height, depth / 2
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH, // width, height
        
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH - (BODY_DEPTH / 3.), // width advance convergent, height, depth / 2
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH - (BODY_DEPTH / 3.),
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH,

        // between A
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH / 3.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 3.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH - (BODY_DEPTH / 3.),

        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH - (BODY_DEPTH / 3.),
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 3.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH - (BODY_DEPTH / 3.),
        
        // upside (red)
        0., 0., 0.,
        BODY_WIDTH, 0., 0.,
        0., 0., BODY_DEPTH,

        0., 0., BODY_DEPTH,
        BODY_WIDTH, 0., 0.,
        BODY_WIDTH, 0., BODY_DEPTH,

        // downside (blue)
        0., BODY_HEIGHT, BODY_DEPTH,
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH,
        0., BODY_HEIGHT, 0.,

        0., BODY_HEIGHT, 0.,
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH,
        BODY_WIDTH, BODY_HEIGHT, 0.,

        // CLOSE BETWEEN UP

        // triangle one up side close
        BODY_WIDTH, BODY_HEIGHT, 0.,
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH / 3., 
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 3.,

        // triangle two up side close
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH - BODY_DEPTH / 3.,
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH - BODY_DEPTH / 3., 

        // close between one & two
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH / 3., 
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH - BODY_DEPTH / 3.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 3.,
        
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 3.,
        BODY_WIDTH, BODY_HEIGHT, BODY_DEPTH - BODY_DEPTH / 3.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH - BODY_DEPTH / 3.,

        // CLOSE BETWEEN DOWN

        // triangle one down side close
        BODY_WIDTH, 0., 0.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH / 3.,
        BODY_WIDTH, 0., BODY_DEPTH / 3., 

        // triangle two down side close
        BODY_WIDTH, 0., BODY_DEPTH,
        BODY_WIDTH, 0., BODY_DEPTH - BODY_DEPTH / 3., 
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH - BODY_DEPTH / 3.,

        // close between one & two
        BODY_WIDTH, 0., BODY_DEPTH / 3., 
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH / 3.,       
        BODY_WIDTH, 0., BODY_DEPTH - BODY_DEPTH / 3.,
      
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH / 3.,
        BODY_WIDTH + BODY_FRONTAL_WIDTH_OFFSET, 0., BODY_DEPTH - BODY_DEPTH / 3.,
        BODY_WIDTH, 0., BODY_DEPTH - BODY_DEPTH / 3.,

        // tail convergent one
        0., 0., BODY_DEPTH,
        -BODY_BACK_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH - BODY_BACK_DEPTH_OFFSET,
        -BODY_BACK_WIDTH_OFFSET, 0., BODY_DEPTH - BODY_BACK_DEPTH_OFFSET,
        
        0., BODY_HEIGHT, BODY_DEPTH,
        -BODY_BACK_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH - BODY_BACK_DEPTH_OFFSET,
        0., 0., BODY_DEPTH,

        // tail convergent two
        -BODY_BACK_WIDTH_OFFSET, 0., BODY_DEPTH / 2.,
        -BODY_BACK_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 2.,
        0., 0., 0.,
        
        0., BODY_HEIGHT, 0.,
        0., 0., 0.,
        -BODY_BACK_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH / 2.,
        
        // tail close triangle up
        0., BODY_HEIGHT, 0.,
        -BODY_BACK_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH -  BODY_BACK_DEPTH_OFFSET,
        0., BODY_HEIGHT, BODY_DEPTH -  BODY_BACK_DEPTH_OFFSET,
        
        0., BODY_HEIGHT, BODY_DEPTH -  BODY_BACK_DEPTH_OFFSET,
        -BODY_BACK_WIDTH_OFFSET, BODY_HEIGHT, BODY_DEPTH -  BODY_BACK_DEPTH_OFFSET,
        0., BODY_HEIGHT, BODY_DEPTH,

        // tail close triangle down
        0., 0., 0.,
        0., 0., BODY_DEPTH -  BODY_BACK_DEPTH_OFFSET,
        -BODY_BACK_WIDTH_OFFSET, 0., BODY_DEPTH -  BODY_BACK_DEPTH_OFFSET,
        
        0., 0., BODY_DEPTH - BODY_BACK_DEPTH_OFFSET,
        0., 0., BODY_DEPTH,
        -BODY_BACK_WIDTH_OFFSET, 0., BODY_DEPTH -  BODY_BACK_DEPTH_OFFSET,
    ];

    body_data
}

pub fn get_leg_data(leg_position: &LegType) -> Vec<Vec<f32>> {

    let upper_leg_width: f32;
    let upper_leg_big_height: f32;
    let upper_leg_small_height: f32;
    let joint_leg_width: f32;
    let joint_leg_big_height: f32;
    let joint_leg_small_height: f32;
    let base_leg_width: f32;
    let base_leg_height: f32;
    let upper_leg_depth: f32;
    let joint_leg_depth: f32;
    let base_leg_depth: f32;
    
    match leg_position {       
        LegType::Frontal => {
            upper_leg_width = FRONTAL_UPPER_LEG_WIDTH;
            upper_leg_big_height = FRONTAL_UPPER_LEG_BIG_HEIGHT;
            upper_leg_small_height = FRONTAL_UPPER_LEG_SMALL_HEIGHT;
            
            joint_leg_width = FRONTAL_JOINT_LEG_WIDTH;
            joint_leg_big_height = FRONTAL_JOINT_LEG_BIG_HEIGHT;
            joint_leg_small_height = FRONTAL_JOINT_LEG_SMALL_HEIGHT;
            
            base_leg_width = FRONTAL_BOTTOM_LEG_WIDTH;
            base_leg_height = FRONTAL_BOTTOM_LEG_HEIGHT;
            
            upper_leg_depth = FRONTAL_UPPER_LEG_DEPTH;
            joint_leg_depth = FRONTAL_JOINT_LEG_DEPTH;
            base_leg_depth = FRONTAL_BASE_LEG_DEPTH;
        },
        LegType::Back => {
            upper_leg_width = BACK_UPPER_LEG_WIDTH;
            upper_leg_big_height = BACK_UPPER_LEG_BIG_HEIGHT;
            upper_leg_small_height = BACK_UPPER_LEG_SMALL_HEIGHT;
            
            joint_leg_width = BACK_JOINT_LEG_WIDTH;
            joint_leg_big_height = BACK_JOINT_LEG_BIG_HEIGHT;
            joint_leg_small_height = BACK_JOINT_LEG_SMALL_HEIGHT;
            
            base_leg_width = BACK_BASE_LEG_WIDTH;
            base_leg_height = BACK_BASE_LEG_HEIGHT;
            
            upper_leg_depth = BACK_UPPER_LEG_DEPTH;
            joint_leg_depth = BACK_JOINT_LEG_DEPTH;
            base_leg_depth = BACK_BASE_LEG_DEPTH;
        },
        LegType::Middle => {
            upper_leg_width = MIDDLE_UPPER_LEG_WIDTH;
            upper_leg_big_height = MIDDLE_UPPER_LEG_BIG_HEIGHT;
            upper_leg_small_height = MIDDLE_UPPER_LEG_SMALL_HEIGHT;
            
            joint_leg_width = MIDDLE_JOINT_LEG_WIDTH;
            joint_leg_big_height = MIDDLE_JOINT_LEG_BIG_HEIGHT;
            joint_leg_small_height = MIDDLE_JOINT_LEG_SMALL_HEIGHT;
            
            base_leg_width = MIDDLE_BASE_LEG_WIDTH;
            base_leg_height = MIDDLE_BASE_LEG_HEIGHT;
            
            upper_leg_depth = MIDDLE_UPPER_LEG_DEPTH;
            joint_leg_depth = MIDDLE_JOINT_LEG_DEPTH;
            base_leg_depth = MIDDLE_BASE_LEG_DEPTH;
        }
    }

    let upper_leg_data = get_upper_or_middle_leg_data(
        upper_leg_width,
        upper_leg_big_height,
        upper_leg_small_height,
        upper_leg_depth
    );
    let middle_leg_data = get_upper_or_middle_leg_data(
        joint_leg_width,
        joint_leg_big_height,
        joint_leg_small_height,
        joint_leg_depth
    );
    let bottom_leg_data = get_bottom_leg_data(
        base_leg_width, 
        base_leg_height, 
        base_leg_depth
    );

    vec![upper_leg_data, middle_leg_data, bottom_leg_data]
}