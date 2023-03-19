pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec4 aColor;

    uniform mat4 uMatrix;

    varying vec4 vColor;

    void main() {
        //float zToDivideBy = 1.0 + aPosition.z; // fudge factor
        //gl_Position = uMatrix * vec4(aPosition.xy / zToDivideBy, aPosition.zw);
        gl_Position = uMatrix * aPosition;

        vColor = aColor;
    }
"#;





