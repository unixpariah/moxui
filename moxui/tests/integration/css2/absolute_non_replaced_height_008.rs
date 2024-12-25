use crate::WgpuCtx;
use calc_units::Units;
use moxui::tree::{self, Config};

pub fn absolute_non_replaced_height_008(wgpu_ctx: &WgpuCtx) -> tree::Tree {
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
                    item.set_content("Test passes if a blue rectangle is below an orange square.")
                })
                .add_child(|item| {
                    item.set_size(Units::In(1.0), Units::In(3.0))
                        .set_background_color(0.0, 0.0, 1.0, 1.0)
                        .set_position(moxui::rectangle::Position::Relative)
                        .add_child(|item| {
                            item.set_background_color(1.0, 0.647, 0.0, 1.0)
                                .set_position(moxui::rectangle::Position::Absolute)
                                .set_size(Units::Perc(100.0), Units::In(1.0))
                                .set_coordinates(
                                    Units::Auto,
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
                        })
                })
        },
    )
    .finish()
}
