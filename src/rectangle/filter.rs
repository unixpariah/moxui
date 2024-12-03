use super::Rectangle;

// Name              | Implemented by Struct | Implemented by Shader
// ------------------|-----------------------|-----------------------
// opacity           | [x]                   | [x]
// blur              | [x]                   | [ ]
// brightness        | [x]                   | [x]
// contrast          | [x]                   | [x]
// grayscale         | [x]                   | [x]
// invert            | [x]                   | [x]
// sepia             | [x]                   | [ ]
// saturate          | [x]                   | [x]
// hue-rotate        | [x]                   | [ ]

impl Rectangle {
    pub fn set_sepia(mut self, sepia: f32) -> Self {
        self.sepia = sepia;
        self
    }

    pub fn set_opacity(mut self, opacity: f32) -> Self {
        self.background_color[3] = opacity;
        self
    }

    pub fn set_blur(mut self, blur: f32) -> Self {
        self.blur = blur;
        self
    }

    pub fn set_brightness(mut self, brightness: f32) -> Self {
        self.brightness = brightness;
        self
    }

    pub fn set_contrast(mut self, contrast: f32) -> Self {
        self.contrast = contrast;
        self
    }

    pub fn set_grayscale(mut self, grayscale: f32) -> Self {
        self.grayscale = grayscale;
        self
    }

    pub fn set_hue_rotate(mut self, hue_rotate: f32) -> Self {
        self.hue_rotate = hue_rotate;
        self
    }

    pub fn set_invert(mut self, invert: f32) -> Self {
        self.invert = invert;
        self
    }

    pub fn set_saturate(mut self, saturate: f32) -> Self {
        self.saturate = saturate;
        self
    }
}
