use super::Rectangle;

// TODO: Implement BorderStyle and make it available to set color per border

pub enum BorderStyle {
    None,
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    Hidden,
}

#[derive(Default)]
pub struct BorderRadius {
    top_left: f32,
    top_right: f32,
    bottom_left: f32,
    bottom_right: f32,
}

impl BorderRadius {
    pub fn to_array(&self) -> [f32; 4] {
        [
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
    }
}

#[derive(Default)]
pub struct BorderSize {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BorderSize {
    pub fn to_array(&self) -> [f32; 4] {
        [self.top, self.right, self.bottom, self.left]
    }
}

pub struct Border {
    pub radius: BorderRadius,
    pub size: BorderSize,
    pub color: [f32; 4],
    pub style: BorderStyle,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            radius: BorderRadius::default(),
            color: [0.0, 0.0, 0.0, 0.0],
            size: BorderSize::default(),
            style: BorderStyle::Solid,
        }
    }
}

impl Rectangle {
    pub fn set_border_size(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.border.size = BorderSize {
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

    pub fn set_border_style(mut self, style: BorderStyle) -> Self {
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
        self.border.radius = BorderRadius {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        };
        self
    }
}
