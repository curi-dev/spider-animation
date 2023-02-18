pub const SHADER: &str = r#"
    precision mediump float;

    varying vec4 vColor;

    void main() {
        gl_FragColor = vColor;
    }
"#;


// pub const SHADER: &str = r#"
//     precision mediump float;

//     uniform sampler2D uImage;

//     varying vec2 vTextCoords;

//     void main() {
//         gl_FragColor = texture2D(uImage, vTextCoords).gbra;
//     }
// "#;