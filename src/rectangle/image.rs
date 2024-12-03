use std::rc::Rc;

//background-attachment: Specifies if the background image scrolls with the page or is fixed (e.g., scroll, fixed, local).
//
//background-origin: Determines the positioning area of the background image (e.g., padding-box, border-box, content-box).
//
//background-clip: Defines the painting area of the background (e.g., border-box, padding-box, content-box, text).
//
//background-blend-mode: Specifies how the background image should blend with the background color or other backgrounds (e.g., multiply, screen, overlay).

enum ImageSize {
    Cover,
    Contain,
    Dimensions(u32, u32),
}

enum ImageRepeat {
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
    Space,
    Round,
}

pub struct Image {
    path: Option<Rc<std::path::Path>>, // This probably should be an actual image or idk yet, maybe
    // a pointer to a texture atlas
    size: ImageSize,
    position: (u32, u32),
    repeat: ImageRepeat,
}
