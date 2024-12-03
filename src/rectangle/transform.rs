use super::Rectangle;

impl Rectangle {
    pub fn set_scale(mut self, x: f32, y: f32) -> Self {
        self.scale = [x, y];
        self
    }

    pub fn set_skew(mut self, x: f32, y: f32) -> Self {
        self.skew = [x, y];
        self
    }

    pub fn set_rotate(mut self, rotation: f32) -> Self {
        self.rotate = rotation;
        self
    }

    pub fn set_translate(mut self, translate: [f32; 2]) -> Self {
        self.translate = translate;
        self
    }
}
