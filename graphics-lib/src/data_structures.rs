use crate::spider::LegType;
use crate::{constants::*, m4};

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
    let bottom_leg_data = get_base_leg_data(
        base_leg_width, 
        base_leg_height, 
        base_leg_depth
    );

    vec![upper_leg_data, middle_leg_data, bottom_leg_data]
}

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

fn get_base_leg_data(width: f32, height: f32, depth: f32) -> Vec<f32> {
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
        
        // bottom face
        0., height, 0.,
        0., height, depth,
        apex.0, apex.1, apex.2, 

        // between front and back side I 
        0., 0., 0.,
        0., 0., depth,
        0., height, 0.,

        0., height, 0.,
        0., 0., depth,
        0., height, depth,
    ];

    bottom_leg_data
}

pub fn get_body_data() -> [f32; 270] {
    let body_data = [
        // side one
        0., 0., 0.,
        0., BODY_HEIGHT, 0.,
        BODY_WIDTH, 0., 0., 
        
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
        
        // downside (red)
        0., 0., 0.,
        BODY_WIDTH, 0., 0.,
        0., 0., BODY_DEPTH,

        0., 0., BODY_DEPTH,
        BODY_WIDTH, 0., 0.,
        BODY_WIDTH, 0., BODY_DEPTH,

        // upside (blue)
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

pub fn get_head_data() -> [f32; 270] {
    let body_data = [
        // side one
        0., 0., 0.,
        0., HEAD_HEIGHT, 0., // height
        HEAD_WIDTH, 0., 0., // width
        
        HEAD_WIDTH, 0., 0., // width
        0., HEAD_HEIGHT, 0., // height
        HEAD_WIDTH, HEAD_HEIGHT, 0., // width, height

        // side two
        0., 0., HEAD_DEPTH, // depth
        HEAD_WIDTH, 0., HEAD_DEPTH, // width, depth
        0., HEAD_HEIGHT, HEAD_DEPTH, // height, depth
        
        HEAD_WIDTH, 0., HEAD_DEPTH, // width, depth
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH, // width, height, depth
        0., HEAD_HEIGHT, HEAD_DEPTH, // height, depth

        // convergent 1 to between A
        HEAD_WIDTH, 0., 0., // width
        HEAD_WIDTH, HEAD_HEIGHT, 0., // width, height
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH / 3., // width advance convergent (width + 10), height, depth / 2
        
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH / 3., // width advance convergent, height, depth / 2
        HEAD_WIDTH, HEAD_HEIGHT, 0.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 3.,

        // convergent 2 to between A
        HEAD_WIDTH, 0., HEAD_DEPTH, // width
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH - (HEAD_DEPTH / 3.), // width advance convergent (width + 10), height, depth / 2
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH, // width, height
        
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH - (HEAD_DEPTH / 3.), // width advance convergent, height, depth / 2
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH - (HEAD_DEPTH / 3.),
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH,

        // between A
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH / 3.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 3.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH - (HEAD_DEPTH / 3.),

        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH - (HEAD_DEPTH / 3.),
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 3.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH - (HEAD_DEPTH / 3.),
        
        // upside (red)
        0., 0., 0.,
        HEAD_WIDTH, 0., 0.,
        0., 0., HEAD_DEPTH,

        0., 0., HEAD_DEPTH,
        HEAD_WIDTH, 0., 0.,
        HEAD_WIDTH, 0., HEAD_DEPTH,

        // downside (blue)
        0., HEAD_HEIGHT, HEAD_DEPTH,
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH,
        0., HEAD_HEIGHT, 0.,

        0., HEAD_HEIGHT, 0.,
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH,
        HEAD_WIDTH, HEAD_HEIGHT, 0.,

        // CLOSE BETWEEN UP

        // triangle one up side close
        HEAD_WIDTH, HEAD_HEIGHT, 0.,
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH / 3., 
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 3.,

        // triangle two up side close
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH - HEAD_DEPTH / 3.,
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH - HEAD_DEPTH / 3., 

        // close between one & two
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH / 3., 
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH - HEAD_DEPTH / 3.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 3.,
        
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 3.,
        HEAD_WIDTH, HEAD_HEIGHT, HEAD_DEPTH - HEAD_DEPTH / 3.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH - HEAD_DEPTH / 3.,

        // CLOSE BETWEEN DOWN

        // triangle one down side close
        HEAD_WIDTH, 0., 0.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH / 3.,
        HEAD_WIDTH, 0., HEAD_DEPTH / 3., 

        // triangle two down side close
        HEAD_WIDTH, 0., HEAD_DEPTH,
        HEAD_WIDTH, 0., HEAD_DEPTH - HEAD_DEPTH / 3., 
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH - HEAD_DEPTH / 3.,

        // close between one & two
        HEAD_WIDTH, 0., HEAD_DEPTH / 3., 
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH / 3.,       
        HEAD_WIDTH, 0., HEAD_DEPTH - HEAD_DEPTH / 3.,
      
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH / 3.,
        HEAD_WIDTH + HEAD_FRONTAL_WIDTH_OFFSET, 0., HEAD_DEPTH - HEAD_DEPTH / 3.,
        HEAD_WIDTH, 0., HEAD_DEPTH - HEAD_DEPTH / 3.,

        // tail convergent one
        0., 0., HEAD_DEPTH,
        -HEAD_BACK_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH - HEAD_BACK_DEPTH_OFFSET,
        -HEAD_BACK_WIDTH_OFFSET, 0., HEAD_DEPTH - HEAD_BACK_DEPTH_OFFSET,
        
        0., HEAD_HEIGHT, HEAD_DEPTH,
        -HEAD_BACK_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH - HEAD_BACK_DEPTH_OFFSET,
        0., 0., HEAD_DEPTH,

        // tail convergent two
        -HEAD_BACK_WIDTH_OFFSET, 0., HEAD_DEPTH / 2.,
        -HEAD_BACK_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 2.,
        0., 0., 0.,
        
        0., HEAD_HEIGHT, 0.,
        0., 0., 0.,
        -HEAD_BACK_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH / 2.,
        
        // tail close triangle up
        0., HEAD_HEIGHT, 0.,
        -HEAD_BACK_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH -  HEAD_BACK_DEPTH_OFFSET,
        0., HEAD_HEIGHT, HEAD_DEPTH -  HEAD_BACK_DEPTH_OFFSET,
        
        0., HEAD_HEIGHT, HEAD_DEPTH -  HEAD_BACK_DEPTH_OFFSET,
        -HEAD_BACK_WIDTH_OFFSET, HEAD_HEIGHT, HEAD_DEPTH -  HEAD_BACK_DEPTH_OFFSET,
        0., HEAD_HEIGHT, HEAD_DEPTH,

        // tail close triangle down
        0., 0., 0.,
        0., 0., HEAD_DEPTH -  HEAD_BACK_DEPTH_OFFSET,
        -HEAD_BACK_WIDTH_OFFSET, 0., HEAD_DEPTH -  HEAD_BACK_DEPTH_OFFSET,
        
        0., 0., HEAD_DEPTH - HEAD_BACK_DEPTH_OFFSET,
        0., 0., HEAD_DEPTH,
        -HEAD_BACK_WIDTH_OFFSET, 0., HEAD_DEPTH -  HEAD_BACK_DEPTH_OFFSET,
    ];

    body_data
}

pub fn get_body_normals() -> [f32; 270] {

    let frontal_normal_vert = nalgebra::Vector3::new(1., 0. , 0.);
    let back_normal_vert = nalgebra::Vector3::new(-1., 0. , 0.);

    // frontal body normal rotation adjusment   
    let frontal_body_convergent_angle = get_frontal_body_convergent_angle();
    let frontal_rotation_mat = nalgebra::Matrix3::from_column_slice(
        &m4::M4::x_rotation( frontal_body_convergent_angle.into() )
    );

    let frontal_negative_rotation_mat = nalgebra::Matrix3::from_column_slice(
        &m4::M4::x_rotation( (- frontal_body_convergent_angle).into() )
    );

    let frontal_normals_mat_positive_rotation = frontal_rotation_mat * frontal_normal_vert;
    let frontal_normals_mat_negative_rotation = frontal_negative_rotation_mat * frontal_normal_vert;
    
    let slice_final_frontal_normals_positive = frontal_normals_mat_positive_rotation.as_slice();
    let slice_final_frontal_normals_negative = frontal_normals_mat_negative_rotation.as_slice();

    // back body normal rotation adjusment
    let back_body_convergent_angle = get_back_body_convergent_angle();
    let back_rotation_mat = nalgebra::Matrix3::from_column_slice(
        &m4::M4::x_rotation( back_body_convergent_angle.into() )
    );

    let back_negative_rotation_mat = nalgebra::Matrix3::from_column_slice(
        &m4::M4::x_rotation( (- back_body_convergent_angle).into() )
    );

    let back_normals_mat_positive_rotation = back_rotation_mat * back_normal_vert;
    let back_normals_mat_negative_rotation = back_negative_rotation_mat * back_normal_vert;
    
    let slice_final_back_normals_positive = back_normals_mat_positive_rotation.as_slice();
    let slice_final_back_normals_negative = back_normals_mat_negative_rotation.as_slice();

    

    let normal_data = [
        // side one
        0., 0., -1.,
        0., 0., -1.,
        0., 0., -1.,
        0., 0., -1.,
        0., 0., -1.,
        0., 0., -1.,

        // side two
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,

        // convergent 1 to between A [has to have an rotation angle]
        slice_final_frontal_normals_positive[0], slice_final_frontal_normals_positive[1], slice_final_frontal_normals_positive[2],
        slice_final_frontal_normals_positive[0], slice_final_frontal_normals_positive[1], slice_final_frontal_normals_positive[2],
        slice_final_frontal_normals_positive[0], slice_final_frontal_normals_positive[1], slice_final_frontal_normals_positive[2],
        
        slice_final_frontal_normals_positive[0], slice_final_frontal_normals_positive[1], slice_final_frontal_normals_positive[2],
        slice_final_frontal_normals_positive[0], slice_final_frontal_normals_positive[1], slice_final_frontal_normals_positive[2],
        slice_final_frontal_normals_positive[0], slice_final_frontal_normals_positive[1], slice_final_frontal_normals_positive[2],
        
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,
        
        // convergent 2 to between A [[has to have an rotation angle]]
        slice_final_frontal_normals_negative[0], slice_final_frontal_normals_negative[1], slice_final_frontal_normals_negative[2],
        slice_final_frontal_normals_negative[0], slice_final_frontal_normals_negative[1], slice_final_frontal_normals_negative[2],
        slice_final_frontal_normals_negative[0], slice_final_frontal_normals_negative[1], slice_final_frontal_normals_negative[2],
        
        slice_final_frontal_normals_negative[0], slice_final_frontal_normals_negative[1], slice_final_frontal_normals_negative[2],
        slice_final_frontal_normals_negative[0], slice_final_frontal_normals_negative[1], slice_final_frontal_normals_negative[2],
        slice_final_frontal_normals_negative[0], slice_final_frontal_normals_negative[1], slice_final_frontal_normals_negative[2],
        
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,
        // 1., 0., 0.,

        // between A
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,

        // downside
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // upside
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // triangle one up side close
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // triangle two up side close
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // close between one & two
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // triangle one down side close
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // triangle two down side close
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // close between one & two
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // tail convergent one
        slice_final_back_normals_positive[0], slice_final_back_normals_positive[1], slice_final_back_normals_positive[2],
        slice_final_back_normals_positive[0], slice_final_back_normals_positive[1], slice_final_back_normals_positive[2],
        slice_final_back_normals_positive[0], slice_final_back_normals_positive[1], slice_final_back_normals_positive[2],
        
        slice_final_back_normals_positive[0], slice_final_back_normals_positive[1], slice_final_back_normals_positive[2],
        slice_final_back_normals_positive[0], slice_final_back_normals_positive[1], slice_final_back_normals_positive[2],
        slice_final_back_normals_positive[0], slice_final_back_normals_positive[1], slice_final_back_normals_positive[2],
        
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,

        // tail convergent two
        slice_final_back_normals_negative[0], slice_final_back_normals_negative[1], slice_final_back_normals_negative[2],
        slice_final_back_normals_negative[0], slice_final_back_normals_negative[1], slice_final_back_normals_negative[2],
        slice_final_back_normals_negative[0], slice_final_back_normals_negative[1], slice_final_back_normals_negative[2],
        
        slice_final_back_normals_negative[0], slice_final_back_normals_negative[1], slice_final_back_normals_negative[2],
        slice_final_back_normals_negative[0], slice_final_back_normals_negative[1], slice_final_back_normals_negative[2],
        slice_final_back_normals_negative[0], slice_final_back_normals_negative[1], slice_final_back_normals_negative[2],
        
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,
        // -1., 0., 0.,

        // tail close triangle up
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // tail close triangle down
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
    ];

    normal_data

}

pub fn get_head_normals() -> [f32; 270] {
    let normal_data = [
        // side one
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,

        // side two
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,

        // convergent 1 to between A
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        
        // convergent 2 to between A
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,

        // between A
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,

        // downside
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // upside
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // triangle one up side close
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // triangle two up side close
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // close between one & two
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // triangle one down side close
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // triangle two down side close
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // close between one & two
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // tail convergent one
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,

        // tail convergent two
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,
        0., 0., 1.,

        // tail close triangle up
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // tail close triangle down
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
    ];

    normal_data
}

pub fn get_leg_upper_and_joint_normals() -> [f32; 108] {

    let normals_data = [
        // front side face 
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // back side face
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // upper face
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // bottom face
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // between front and back side I
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,

        // between front and back side II
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
        1., 0., 0.,
    ];

    normals_data
}

pub fn get_leg_base_normals() -> [f32; 54] {
    let normals_data = [
        // front side face
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        
        // back side face
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,

        // upper face
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        // bottom face
        0., -1., 0.,
        0., -1., 0.,
        0., -1., 0.,
        
        // between front and back side I 
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,

        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
    ];

    normals_data
}

pub fn get_floor_data() -> [f32; 18] {
    
    let floor_data: [f32; 18] = [
        -200., 0., 0.,
        1000., 0., 0.,
        -200., 0., -500.,
        
        -200., 0., -500.,
        1000., 0., 0.,
        1000., 0., -500.,
    ];

    floor_data
}

pub fn get_floor_normals() -> [f32; 18] {


    let normals_data = [
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
        0., 1., 0.,
    ];

    normals_data
}


// COLORS [NOT USING]
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

