pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec4 aColor;

    uniform mat4 uMatrix;

    varying vec4 vColor;

    void main() {
        gl_Position = uMatrix * aPosition;

        vColor = aColor;
    }
"#;


// pub const SHADER: &str = r#"
//     attribute vec2 aPosition;
//     attribute vec2 aTextCoords;
    
//     uniform mat3 uTranslation;
//     uniform mat3 uRotation;
//     uniform vec2 uResolution;

//     varying vec2 vTextCoords;

//     void main() {
//         vec2 clipSpace = (aPosition / uResolution) * 2.0 - 1.0;
        
//         vec2 translatedPositions = (uTranslation * vec3(clipSpace, 1)).xy;
//         vec2 rotatedPositions = (uRotation * vec3(translatedPositions, 1)).xy;
 
//         gl_Position = vec4(rotatedPositions, 0, 1);
//         gl_PointSize = 5.;
        
//         vTextCoords = aTextCoords;
//     }
// "#;


