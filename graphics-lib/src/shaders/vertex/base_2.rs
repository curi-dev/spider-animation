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

// vec3 clipSpace = (aPosition.xyz / vec3(uResolution, 0)) * 2.0 - 1.0;
// vec3 clipSpace = (aPosition.xyz / uResolution.xyx) * 2. - 1.;

// gl_Position = vec4(clipSpace, aPosition.w);