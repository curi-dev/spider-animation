

pub struct MatrixStack {
    pub stack: Vec<[f32; 16]>,

}

impl MatrixStack {

    pub fn pop(&mut self) -> Option<[f32; 16]> {
        self.stack.pop()
    }

    pub fn push(&mut self, matrix: [f32; 16]) {
        self.stack.push(matrix)
    }

    pub fn reset(&mut self) {
        self.stack.drain(..);
    }
}