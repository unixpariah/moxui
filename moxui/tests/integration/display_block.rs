use crate::WgpuCtx;
use calc_macro::calc;
use calc_units::Units;
use moxui::tree::{self, Config};

pub fn display_block(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    let config = &wgpu_ctx.surface_config;

    moxui::tree::Tree::new(
        &wgpu_ctx.device,
        &Config {
            width: config.width as f32,
            height: config.height as f32,
            format: config.format,
            dpi: 94.1,
        },
        |surface| {
            surface
                .set_background_color(0.0, 0.0, 0.0, 0.0)
                .add_child(|item| {
                    item.set_background_color(0.0, 0.0, 1.0, 1.0)
                        .add_child(|item| {
                            item.set_background_color(0.0, 1.0, 0.0, 1.0)
                                .set_margin(
                                    Units::Px(50.0),
                                    Units::Px(0.0),
                                    Units::Px(0.0),
                                    calc!("50%" - 25px),
                                )
                                .set_size(Units::Px(50.0), Units::Px(50.0))
                        })
                })
                .add_child(|item| {
                    item.set_background_color(1.0, 0.0, 0.0, 1.0)
                        .set_size(Units::Auto, Units::Px(50.0))
                })
                .add_child(|item| item.set_background_color(0.0, 0.0, 1.0, 1.0))
                .add_child(|item| {
                    item.set_background_color(0.0, 1.0, 0.0, 1.0)
                        .set_size(Units::Perc(50.0), Units::Px(50.0))
                })
                .add_child(|item| {
                    item.set_background_color(1.0, 0.0, 0.0, 1.0)
                        .set_size(Units::Perc(50.0), Units::Px(50.0))
                        .set_margin(
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(50.0),
                            Units::Px(0.0),
                        )
                })
                .add_child(|item| {
                    item.set_background_color(0.5, 0.5, 0.0, 1.0)
                        .set_margin(
                            Units::Px(25.0),
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(0.0),
                        )
                        .add_child(|item| {
                            item.set_background_color(0.0, 1.0, 0.0, 1.0)
                                .set_size(Units::Px(50.0), Units::Px(50.0))
                        })
                        .add_child(|item| {
                            item.set_background_color(0.0, 0.0, 1.0, 1.0)
                                .set_size(Units::Px(50.0), Units::Px(50.0))
                        })
                })
        },
    )
    .finish()
}
