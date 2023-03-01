pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec4 aColor;

    uniform mat4 uMatrix;

    varying vec4 vColor;

    //out vec4 outPositions;

    void main() {
        //float zToDivideBy = 1.0 + aPosition.z;
        //float zToDivideBy = 1.0;
        //gl_Position = uMatrix * vec4(aPosition.xy / zToDivideBy, aPosition.zw);
        gl_Position = uMatrix * aPosition;

        vColor = aColor;

        //outPositions = gl_Position;
    }
"#;





