use crate::rectangle::{self};
use calc_units::Units;

use super::Node;

impl Node {
    pub fn set_coordinates(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.top = top;
        self.style.right = right;
        self.style.bottom = bottom;
        self.style.left = left;
        self
    }

    pub fn set_position(mut self, position: rectangle::Position) -> Self {
        self.style.position = position;
        self
    }

    pub fn set_display(mut self, display: rectangle::Display) -> Self {
        self.style.display = display;
        self
    }

    pub fn set_size(mut self, width: Units, height: Units) -> Self {
        self.style.width = width;
        self.style.height = height;
        self
    }

    pub fn set_box_sizing(mut self, box_sizing: rectangle::BoxSizing) -> Self {
        self.style.box_sizing = box_sizing;
        self
    }

    pub fn set_padding(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.padding = [top, right, bottom, left];

        self
    }

    pub fn set_background_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.background_color = [r, g, b, a];
        self
    }

    pub fn set_margin(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.margin = [top, right, bottom, left];
        self
    }

    pub fn set_border_size(mut self, top: Units, right: Units, bottom: Units, left: Units) -> Self {
        self.style.border = [top, right, bottom, left];
        self
    }

    pub fn set_border_top_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.top_color = [r, g, b, a];
        self
    }

    pub fn set_border_bottom_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.bottom_color = [r, g, b, a];
        self
    }

    pub fn set_border_left_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.left_color = [r, g, b, a];
        self
    }

    pub fn set_border_right_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.border.right_color = [r, g, b, a];
        self
    }

    pub fn set_border_radius(
        mut self,
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        self.border.radius = [top_left, top_right, bottom_right, bottom_left];
        self
    }

    pub fn set_outline_width(mut self, width: Units) -> Self {
        self.style.outline_width = width;
        self
    }

    pub fn set_outline_offset(mut self, offset: Units) -> Self {
        self.style.outline_offset = offset;
        self
    }

    pub fn set_outline_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.outline.color = [r, g, b, a];
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
