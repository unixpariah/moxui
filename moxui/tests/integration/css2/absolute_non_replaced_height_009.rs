use crate::WgpuCtx;
use calc_units::Units;
use moxui::tree::{self, Config};

pub fn absolute_non_replaced_height_009(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    let config = &wgpu_ctx.surface_config;

    moxui::tree::Tree::new(
        &wgpu_ctx.device,
        &wgpu_ctx.queue,
        &Config {
            width: config.width as f32,
            height: config.height as f32,
            format: config.format,
            dpi: 94.1,
        },
        |surface| {
            surface
                .add_child(|item| {
                    item.set_content(
                        "Test passes if the orange and blue squares have the same height.",
                    )
                })
                .add_child(|item| {
                    item.set_position(moxui::rectangle::Position::Relative)
                        .add_child(|item| {
                            item.set_background_color(0.0, 0.0, 1.0, 1.0)
                                .set_position(moxui::rectangle::Position::Absolute)
                                .set_size(Units::Px(200.0), Units::Auto)
                                .set_coordinates(
                                    Units::Px(25.0),
                                    Units::Px(0.0),
                                    Units::Auto,
                                    Units::Px(0.0),
                                )
                                .set_margin(
                                    Units::Auto,
                                    Units::Px(0.0),
                                    Units::Auto,
                                    Units::Px(0.0),
                                )
                                .set_font_size(Units::Px(100.0))
                                .set_font_color(glyphon::Color::rgb(255, 165, 0))
                                .set_font_family(glyphon::FamilyOwned::Name("Ahem".into()))
                                .set_line_height(Units::Perc(100.0))
                                .set_content("X")
                        })
                })
        },
    )
    .finish()
}
