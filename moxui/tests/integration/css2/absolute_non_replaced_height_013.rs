use crate::WgpuCtx;
use calc_units::Units;
use moxui::tree::{self, Config};

pub fn absolute_non_replaced_height_013(wgpu_ctx: &WgpuCtx) -> tree::Tree {
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
                    item.set_content("Test passes if there is a filled green square and no red.")
                })
                .add_child(|item| {
                    item.set_position(moxui::rectangle::Position::Relative)
                        .set_size(Units::Px(100.0), Units::Px(100.0))
                        .set_background_color(1.0, 0.0, 0.0, 1.0)
                        .add_child(|item| {
                            item.set_position(moxui::rectangle::Position::Absolute)
                                .set_coordinates(
                                    Units::Perc(50.0),
                                    Units::Px(0.0),
                                    Units::Perc(50.0),
                                    Units::Px(0.0),
                                )
                                .set_size(Units::Px(100.0), Units::Px(100.0))
                                .set_background_color(0.0, 1.0, 0.0, 1.0)
                                .set_margin(Units::Auto, Units::Auto, Units::Auto, Units::Auto)
                        })
                })
        },
    )
    .finish()
}
