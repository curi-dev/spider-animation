pub const SHADER: &str = r#"
    precision mediump float;

    uniform vec3 uReverseLight;
    uniform vec4 uColor;

    varying vec3 vNormal;

    void main() {
        vec3 normal = normalize(vNormal); // why normalize that (interpolation?)

        float light = dot(normal, uReverseLight); 

        gl_FragColor = uColor;

        gl_FragColor.rgb *= light; 
    }
"#;
