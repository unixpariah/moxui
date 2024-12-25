use glyphon::{
    Buffer, Cache, Color, FontSystem, SwashCache, TextArea, TextAtlas, TextRenderer, Viewport,
};
use wgpu::MultisampleState;

use super::Config;

pub struct TextData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub buffer: Buffer,
    pub color: Color,
}

pub struct Text {
    pub buffer: Buffer,
    pub font_system: FontSystem,
}

impl Text {
    pub fn extents(&self) -> (f32, f32) {
        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0.0), |(width, total_lines), run| {
                (run.line_w.max(width), total_lines + 1.0)
            });

        (width, total_lines * self.buffer.metrics().line_height)
    }
}

pub struct TextContext {
    pub swash_cache: glyphon::SwashCache,
    pub viewport: glyphon::Viewport,
    pub atlas: glyphon::TextAtlas,
    pub renderer: glyphon::TextRenderer,
}

impl TextContext {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, config: &Config) -> Self {
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let mut viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, config.format);
        let renderer = TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);

        viewport.update(
            queue,
            glyphon::Resolution {
                width: config.width as u32,
                height: config.height as u32,
            },
        );

        Self {
            swash_cache,
            viewport,
            atlas,
            renderer,
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
        text: Vec<TextArea>,
    ) {
        let mut font_system = FontSystem::new();

        self.renderer
            .prepare(
                device,
                queue,
                &mut font_system,
                &mut self.atlas,
                &self.viewport,
                text,
                &mut self.swash_cache,
            )
            .unwrap();

        self.renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .unwrap();
    }
}
