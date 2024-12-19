use crate::WgpuCtx;
use calc_macro::calc;
use calc_units::Units;
use moxui::tree::{self, Config};

pub fn scroll(wgpu_ctx: &WgpuCtx) -> tree::Tree {
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
                    item.set_display(moxui::rectangle::Display::InlineBlock)
                        .set_background_color(0.0, 0.0, 1.0, 1.0)
                        .set_size(Units::Perc(150.0), Units::Px(50.0))
                        .set_border_size(
                            Units::Px(5.0),
                            Units::Px(5.0),
                            Units::Px(5.0),
                            Units::Px(5.0),
                        )
                        .set_border_top_color(1.0, 0.0, 0.0, 1.0)
                        .set_border_bottom_color(1.0, 1.0, 0.0, 1.0)
                        .set_border_left_color(0.0, 1.0, 0.0, 1.0)
                        .set_border_right_color(1.0, 0.0, 1.0, 1.0)
                })
                .add_child(|item| {
                    item.set_display(moxui::rectangle::Display::InlineBlock)
                        .set_background_color(1.0, 0.0, 0.0, 1.0)
                        .set_size(Units::Px(50.0), Units::Perc(150.0))
                })
                .add_child(|item| {
                    item.set_display(moxui::rectangle::Display::InlineBlock)
                        .set_background_color(0.0, 1.0, 0.0, 1.0)
                        .set_size(Units::Px(150.0), Units::Px(150.0))
                })
                .add_child(|item| {
                    item.set_position(moxui::rectangle::Position::Sticky)
                        .set_display(moxui::rectangle::Display::InlineBlock)
                        .set_background_color(1.0, 1.0, 0.0, 1.0)
                        .set_size(Units::Px(150.0), Units::Px(150.0))
                        .set_coordinates(Units::Px(10.0), Units::Auto, Units::Auto, Units::Auto)
                        .set_margin(
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(50.0),
                        )
                })
                .add_child(|item| {
                    item.set_position(moxui::rectangle::Position::Fixed)
                        .set_background_color(1.0, 1.0, 0.0, 1.0)
                        .set_size(Units::Px(150.0), Units::Px(150.0))
                        .set_margin(
                            calc!(50vh - 75px),
                            Units::Px(0.0),
                            Units::Px(0.0),
                            calc!(50vw - 75px),
                        )
                })
        },
    )
    .finish()
}
