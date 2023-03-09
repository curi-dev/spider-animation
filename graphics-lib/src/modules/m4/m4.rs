
pub struct M4 {}

impl M4 {
    pub fn projection(width: f32, height: f32, depth: f32) -> [f32; 16] {
        let projection_mat = [
            2. / width, 0., 0., 0.,
            0., -2. / height, 0., 0.,
            0., 0., 2. / depth, 0.,
           -1., 1., 0., 1.,
        ];

        projection_mat
    }

    pub fn perspective(field_of_view_in_radians: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
        let f = ((3.14 * 0.5 - 0.5 * field_of_view_in_radians) as f64).tan();
        let range_inv = 1. / (near - far);

        let perspective_mat = [
            f as f32 / aspect, 0., 0., 0.,
            0., f as f32, 0., 0.,
            0., 0., (near + far) * range_inv, -1.,
            0., 0., near * far * range_inv * 2., 0. 
        ];

        perspective_mat
    }

    pub fn identity() -> [f32; 16] {        
        let identity_mat = [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1. 
        ];

        identity_mat
    }

    pub fn translate_3_d(pre_matrix: [f32; 16], translation_mat: [f32; 16]) -> [f32; 16] {
        M4::multiply_mat(pre_matrix, translation_mat)  
    }

    pub fn translation(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
        let translation_mat = [
            1.,  0.,  0.,  0.,
            0.,  1.,  0.,  0.,
            0.,  0.,  1.,  0.,
            tx, ty, tz, 1.,
        ];

        translation_mat
    }

    pub fn scale_3_d(pre_matrix: [f32; 16], scale_mat: [f32; 16]) -> [f32; 16] {
        M4::multiply_mat(pre_matrix, scale_mat)
    }

    pub fn scaling(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
        let scale_mat = [
            sx, 0.,  0.,  0.,
            0., sy,  0.,  0.,
            0.,  0., sz,  0.,
            0.,  0.,  0.,  1.,
        ];

        scale_mat
    }

    pub fn x_rotate_3_d( pre_matrix: [f32; 16], x_rotation_mat: [f32; 16]) -> [f32; 16] { // convert to radians
        M4::multiply_mat(pre_matrix, x_rotation_mat, )
    }

    pub fn x_rotation(angle_in_radians: f64) -> [f32; 16] {
        let cos = angle_in_radians.cos() as f32;
        let sin = angle_in_radians.sin() as f32;
        
        let x_rotation_mat = [
            1., 0., 0., 0.,
            0., cos, sin, 0.,
            0., -sin, cos, 0.,
            0., 0., 0., 1.,
        ];

        x_rotation_mat
    }

    pub fn y_rotate_3_d(pre_matrix: [f32; 16], y_rotation_mat: [f32; 16]) -> [f32; 16] { // convert to radians
        M4::multiply_mat(pre_matrix, y_rotation_mat)
    }

    pub fn y_rotation(angle_in_radians: f64) -> [f32; 16] {
        let cos = angle_in_radians.cos() as f32;
        let sin = angle_in_radians.sin() as f32;
        
        let y_rotation_mat = [
            cos, 0., -sin, 0.,
            0., 1., 0., 0.,
            sin, 0., cos, 0.,
            0., 0., 0., 1.,
        ];

        y_rotation_mat
    }

    pub fn z_rotate_3_d(pre_matrix: [f32; 16], z_rotation_mat: [f32; 16]) -> [f32; 16] { // convert to radians       
        M4::multiply_mat(pre_matrix, z_rotation_mat)
    }

    pub fn z_rotation(angle_in_radians: f64) -> [f32; 16] {
        let cos = angle_in_radians.cos() as f32;
        let sin = angle_in_radians.sin() as f32;
        
        let z_rotation_mat = [
            cos, sin, 0., 0.,
           -sin, cos, 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
         ];

        z_rotation_mat
    }

    pub fn multiply_mat(mat_a: [f32; 16], mat_b: [f32; 16]) -> [f32; 16] {
        let b00 = mat_b[0 * 4 + 0];
        let b01 = mat_b[0 * 4 + 1];
        let b02 = mat_b[0 * 4 + 2];
        let b03 = mat_b[0 * 4 + 3];
        let b10 = mat_b[1 * 4 + 0];
        let b11 = mat_b[1 * 4 + 1];
        let b12 = mat_b[1 * 4 + 2];
        let b13 = mat_b[1 * 4 + 3];
        let b20 = mat_b[2 * 4 + 0];
        let b21 = mat_b[2 * 4 + 1];
        let b22 = mat_b[2 * 4 + 2];
        let b23 = mat_b[2 * 4 + 3];
        let b30 = mat_b[3 * 4 + 0];
        let b31 = mat_b[3 * 4 + 1];
        let b32 = mat_b[3 * 4 + 2];
        let b33 = mat_b[3 * 4 + 3];
        let a00 = mat_a[0 * 4 + 0];
        let a01 = mat_a[0 * 4 + 1];
        let a02 = mat_a[0 * 4 + 2];
        let a03 = mat_a[0 * 4 + 3];
        let a10 = mat_a[1 * 4 + 0];
        let a11 = mat_a[1 * 4 + 1];
        let a12 = mat_a[1 * 4 + 2];
        let a13 = mat_a[1 * 4 + 3];
        let a20 = mat_a[2 * 4 + 0];
        let a21 = mat_a[2 * 4 + 1];
        let a22 = mat_a[2 * 4 + 2];
        let a23 = mat_a[2 * 4 + 3];
        let a30 = mat_a[3 * 4 + 0];
        let a31 = mat_a[3 * 4 + 1];
        let a32 = mat_a[3 * 4 + 2];
        let a33 = mat_a[3 * 4 + 3];
 
        return [
            b00 * a00 + b01 * a10 + b02 * a20 + b03 * a30,
            b00 * a01 + b01 * a11 + b02 * a21 + b03 * a31,
            b00 * a02 + b01 * a12 + b02 * a22 + b03 * a32,
            b00 * a03 + b01 * a13 + b02 * a23 + b03 * a33,
            // ------------------------------------------
            b10 * a00 + b11 * a10 + b12 * a20 + b13 * a30,
            b10 * a01 + b11 * a11 + b12 * a21 + b13 * a31,
            b10 * a02 + b11 * a12 + b12 * a22 + b13 * a32,
            b10 * a03 + b11 * a13 + b12 * a23 + b13 * a33,
            // ------------------------------------------
            b20 * a00 + b21 * a10 + b22 * a20 + b23 * a30,
            b20 * a01 + b21 * a11 + b22 * a21 + b23 * a31,
            b20 * a02 + b21 * a12 + b22 * a22 + b23 * a32,
            b20 * a03 + b21 * a13 + b22 * a23 + b23 * a33,
            // ------------------------------------------
            b30 * a00 + b31 * a10 + b32 * a20 + b33 * a30,
            b30 * a01 + b31 * a11 + b32 * a21 + b33 * a31,
            b30 * a02 + b31 * a12 + b32 * a22 + b33 * a32,
            b30 * a03 + b31 * a13 + b32 * a23 + b33 * a33,
        ];
    }
}