pub const SHADER: &str = r#"
    attribute vec2 aPosition;

    uniform vec2 uResolution;

    void main() {
        vec2 clipSpace = (aPosition / uResolution) * 2.0 - 1.0;
    
        gl_Position = vec4(clipSpace, 0, 1);
    }
"#;



