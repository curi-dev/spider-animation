use crate::spider::LegPosition;
use crate::constants::*;

fn get_upper_or_middle_leg_data(width: f32, big_height: f32, small_height: f32, depth: f32) -> [f32; 108] {
    let upper_leg_data: [f32; 108] = [
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

fn get_bottom_leg_data(width: f32, height: f32, depth: f32) -> [f32; 54] {
    let apex = (width, height / 2., depth / 2.);
    
    let bottom_leg_data = [
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

pub fn get_body_colors() -> [u8; 126] {
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
        160, 160, 220,
        160, 160, 220,
        160, 160, 220,
        160, 160, 220,
        160, 160, 220,
        160, 160, 220,

        // convergent 2 to between A
        110, 60, 180,
        110, 60, 180,
        110, 60, 180,
        110, 60, 180,
        110, 60, 180,
        110, 60, 180,

        // between A
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,
        26, 110, 210,

        // up side
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,
        76, 10, 0,

        // down side
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,
        6, 10, 100,
    ];

    colors
}

pub fn get_body_data() -> [f32; 126] {
    let body_data = [
        // side one
        0., 0., 0.,
        0., 17., 0., // height
        20., 0., 0., // width
        
        20., 0., 0., // width
        0., 17., 0., // height
        20., 17., 0., // width, height

        // side two
        0., 0., 20., // depth
        20., 0., 20., // width, depth
        0., 17., 20., // height, depth
        
        20., 0., 20., // width, depth
        20., 17., 20., // width, height, depth
        0., 17., 20., // height, depth

        // convergent 1 to between A
        20., 0., 0., // width
        20., 17., 0., // width, height
        24.5, 0., 20. / 3., // width advance convergent (width + 10), height, depth / 2
        
        24.5, 0., 20. / 3., // width advance convergent, height, depth / 2
        20., 17., 0.,
        24.5, 17., 20. / 3.,

        // convergent 2 to between A
        20., 0., 20., // width
        24.5, 0., 20. - (20. / 3.), // width advance convergent (width + 10), height, depth / 2
        20., 17., 20., // width, height
        
        24.5, 0., 20. - (20. / 3.), // width advance convergent, height, depth / 2
        24.5, 17., 20. - (20. / 3.),
        20., 17., 20.,

        // between A
        24.5, 0., 20. / 3.,
        24.5, 17., 20. / 3.,
        24.5, 0., 20. - (20. / 3.),

        24.5, 0., 20. - (20. / 3.),
        24.5, 17., 20. / 3.,
        24.5, 17., 20. - (20. / 3.),
        
        // down red side
        0., 0., 0.,
        20., 0., 0.,
        0., 0., 20.,

        0., 0., 20.,
        20., 0., 0.,
        20., 0., 20.,

        // up side
        0., 17., 20.,
        20., 17., 20.,
        0., 17., 0.,

        0., 17., 0.,
        20., 17., 20.,
        20., 17., 0.,
    ];

    body_data
}

pub fn get_leg_data(leg_position: &LegPosition) -> ([f32; 108], [f32; 108], [f32; 54]) {

    let frontal_upper_leg_width: f32;
    let frontal_upper_leg_big_height: f32;
    let frontal_upper_leg_small_height: f32;
    let frontal_middle_leg_width: f32;
    let frontal_middle_leg_big_height: f32;
    let frontal_middle_leg_small_height: f32;
    let frontal_bottom_leg_width: f32;
    let frontal_bottom_leg_height: f32;
    let upper_leg_depth: f32;
    let middle_leg_depth: f32;
    let base_leg_depth: f32;
    
    match leg_position {       
        LegPosition::Frontal => {
            frontal_upper_leg_width = FRONTAL_UPPER_LEG_WIDTH;
            frontal_upper_leg_big_height = FRONTAL_UPPER_LEG_BIG_HEIGHT;
            frontal_upper_leg_small_height = FRONTAL_UPPER_LEG_SMALL_HEIGHT;
            frontal_middle_leg_width = FRONTAL_MIDDLE_LEG_WIDTH;
            frontal_middle_leg_big_height = FRONTAL_MIDDLE_LEG_BIG_HEIGHT;
            frontal_middle_leg_small_height = FRONTAL_MIDDLE_LEG_SMALL_HEIGHT;
            frontal_bottom_leg_width = FRONTAL_BOTTOM_LEG_WIDTH;
            frontal_bottom_leg_height = FRONTAL_BOTTOM_LEG_HEIGHT;
            upper_leg_depth = LEG_DEPTH;
            middle_leg_depth = MIDDLE_LEG_DEPTH;
            base_leg_depth = BASE_LEG_DEPTH;
        },
        LegPosition::Back => {
            frontal_upper_leg_width = FRONTAL_UPPER_LEG_WIDTH;
            frontal_upper_leg_big_height = FRONTAL_UPPER_LEG_BIG_HEIGHT;
            frontal_upper_leg_small_height = FRONTAL_UPPER_LEG_SMALL_HEIGHT;
            frontal_middle_leg_width = FRONTAL_MIDDLE_LEG_WIDTH;
            frontal_middle_leg_big_height = FRONTAL_MIDDLE_LEG_BIG_HEIGHT;
            frontal_middle_leg_small_height = FRONTAL_MIDDLE_LEG_SMALL_HEIGHT;
            frontal_bottom_leg_width = FRONTAL_BOTTOM_LEG_WIDTH;
            frontal_bottom_leg_height = FRONTAL_BOTTOM_LEG_HEIGHT;
            upper_leg_depth = LEG_DEPTH;
            middle_leg_depth = MIDDLE_LEG_DEPTH;
            base_leg_depth = BASE_LEG_DEPTH;
        },
        LegPosition::Middle => {
            frontal_upper_leg_width = FRONTAL_UPPER_LEG_WIDTH;
            frontal_upper_leg_big_height = FRONTAL_UPPER_LEG_BIG_HEIGHT;
            frontal_upper_leg_small_height = FRONTAL_UPPER_LEG_SMALL_HEIGHT;
            frontal_middle_leg_width = FRONTAL_MIDDLE_LEG_WIDTH;
            frontal_middle_leg_big_height = FRONTAL_MIDDLE_LEG_BIG_HEIGHT;
            frontal_middle_leg_small_height = FRONTAL_MIDDLE_LEG_SMALL_HEIGHT;
            frontal_bottom_leg_width = FRONTAL_BOTTOM_LEG_WIDTH;
            frontal_bottom_leg_height = FRONTAL_BOTTOM_LEG_HEIGHT;
            upper_leg_depth = LEG_DEPTH;
            middle_leg_depth = MIDDLE_LEG_DEPTH;
            base_leg_depth = BASE_LEG_DEPTH;
        }       
    }

    let upper_leg_data = get_upper_or_middle_leg_data(
        frontal_upper_leg_width,
        frontal_upper_leg_big_height,
        frontal_upper_leg_small_height,
        upper_leg_depth
    );
    let middle_leg_data = get_upper_or_middle_leg_data(
        frontal_middle_leg_width,
        frontal_middle_leg_big_height,
        frontal_middle_leg_small_height,
        middle_leg_depth
    );
    let bottom_leg_data = get_bottom_leg_data(
        frontal_bottom_leg_width, 
        frontal_bottom_leg_height, 
        base_leg_depth
    );

    (upper_leg_data, middle_leg_data, bottom_leg_data)
}