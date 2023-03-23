pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec3 aNormal;

    uniform mat4 uMatrix;
    uniform mat4 uNormalMatrix;
   
    varying vec3 vNormal;

    void main() {
        gl_Position = uMatrix * aPosition;

        vec3 updatedNormal = mat3(uNormalMatrix) * aNormal;

        vNormal = updatedNormal;
    }
"#;





