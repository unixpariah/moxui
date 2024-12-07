use crate::rectangle;

use super::Node;

// Name              | Implemented by Struct | Implemented by Shader
// ------------------|-----------------------|-----------------------
// opacity           | [x]                   | [x]
// blur              | [ ]                   | [ ]
// brightness        | [x]                   | [x]
// contrast          | [x]                   | [x]
// grayscale         | [x]                   | [x]
// invert            | [x]                   | [x]
// sepia             | [x]                   | [x]
// saturate          | [x]                   | [x]
// hue-rotate        | [x]                   | [ ]

impl Node {
    pub fn set_display(mut self, display: rectangle::Display) -> Self {
        self.style.display = display;
        self
    }

    pub fn set_size(mut self, width: rectangle::Units, height: rectangle::Units) -> Self {
        self.style.width = Some(width);
        self.style.height = Some(height);
        self
    }

    pub fn set_box_sizing(mut self, box_sizing: rectangle::BoxSizing) -> Self {
        self.style.box_sizing = box_sizing;
        self
    }

    pub fn set_padding(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.padding = rectangle::Spacing {
            top,
            right,
            bottom,
            left,
        };
        self
    }

    pub fn set_background_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.background_color = [r, g, b, a];
        self
    }

    pub fn set_margin(
        mut self,
        top: rectangle::Units,
        right: rectangle::Units,
        bottom: rectangle::Units,
        left: rectangle::Units,
    ) -> Self {
        self.style.margin = [top, right, bottom, left];
        self
    }

    pub fn set_border_size(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.border.size = rectangle::BorderSize {
            top,
            right,
            bottom,
            left,
        };
        self
    }

    pub fn set_border_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.color = [r, g, b, a];
        self
    }

    pub fn set_border_style(mut self, style: rectangle::BorderStyle) -> Self {
        self.border.style = style;
        self
    }

    pub fn set_border_radius(
        mut self,
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        self.border.radius = rectangle::BorderRadius {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        };
        self
    }

    pub fn set_outline_width(mut self, width: f32) -> Self {
        self.outline.width = width;
        self
    }

    pub fn set_outline_offset(mut self, offset: f32) -> Self {
        self.outline.offset = offset;
        self
    }

    pub fn set_outline_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.outline.color = [r, g, b, a];
        self
    }

    pub fn set_outline_style(mut self, style: rectangle::OutlineStyle) -> Self {
        self.outline.style = style;
        self
    }

    pub fn set_sepia(mut self, sepia: f32) -> Self {
        self.sepia = sepia;
        self
    }

    pub fn set_opacity(mut self, opacity: f32) -> Self {
        self.background_color[3] = opacity;
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
