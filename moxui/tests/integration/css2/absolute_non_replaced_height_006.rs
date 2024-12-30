use crate::WgpuCtx;
use calc_units::Units;
use moxui::tree::{self, Config};

pub fn absolute_non_replaced_height_006(wgpu_ctx: &WgpuCtx) -> tree::Tree {
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
                        "Test passes if there is one and only one blue rectangle inside a hollow black square and if such black square does not have a vertical scrollbar."
                    )
                })
                .add_child(|item| {
                    item.set_border_size(
                        Units::Px(10.0),
                        Units::Px(10.0),
                        Units::Px(10.0),
                        Units::Px(10.0),
                    )
                    .set_border_top_color(1.0, 1.0, 1.0, 1.0)
                    .set_border_bottom_color(1.0, 1.0, 1.0, 1.0)
                    .set_border_left_color(1.0, 1.0, 1.0, 1.0)
                    .set_border_right_color(1.0, 1.0, 1.0, 1.0)
                    .set_size(Units::Px(300.0), Units::Px(300.0))
                    .set_position(moxui::rectangle::Position::Relative)
                    .add_child(|item| {
                        item
.set_position(moxui::rectangle::Position::Absolute)
                        .set_background_color(0.0, 0.0, 1.0, 1.0)
                        .set_size(Units::Perc(50.0), Units::Px(150.0))
                        .set_margin(
                            Units::Px(50.0),
                            Units::Px(0.0),
                            Units::Px(50.0),
                            Units::Px(0.0),
                        )
                        .set_coordinates(
                            Units::Px(50.0),
                            Units::Px(0.0),
                            Units::Px(0.0),
                            Units::Px(0.0),
                        )
                    })
                    .add_child(|item| {
                        item.set_position(moxui::rectangle::Position::Absolute)
                                .set_background_color(0.0, 0.0, 1.0, 1.0)
                            .set_size(Units::Perc(50.0), Units::Px(150.0))
                            .set_margin(
                                Units::Px(50.0),
                                Units::Px(0.0),
                                Units::Px(50.0),
                                Units::Px(0.0),
                            )
                            .set_coordinates(
                                Units::Px(50.0),
                                Units::Px(0.0),
                                Units::Px(0.0),
                                Units::Px(0.0),
                            )
                    })
                })
        },
    )
    .finish()
}
