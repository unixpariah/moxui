use crate::WgpuCtx;
use calc_units::Units;
use moxui::{rectangle, tree};

pub fn position_absolute(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    moxui::tree::Tree::new(&wgpu_ctx.device, &wgpu_ctx.surface_config, |surface| {
        surface
            .set_background_color(0.0, 0.0, 0.0, 0.0)
            .add_child(|item| {
                item.set_position(rectangle::Position::Absolute)
                    .set_coordinates(
                        Units::Px(150.0),
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(100.0),
                    )
                    .set_display(rectangle::Display::InlineBlock)
                    .set_background_color(0.0, 0.0, 1.0, 1.0)
                    .set_size(Units::Px(50.0), Units::Px(50.0))
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::InlineBlock)
                    .set_background_color(0.0, 1.0, 0.0, 1.0)
                    .set_size(Units::Px(50.0), Units::Px(50.0))
            })
            .add_child(|item| {
                item.set_position(rectangle::Position::Absolute)
                    .set_coordinates(
                        Units::Px(200.0),
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(100.0),
                    )
                    .set_display(moxui::rectangle::Display::InlineBlock)
                    .set_background_color(0.2, 0.2, 1.0, 1.0)
                    .set_size(Units::Px(50.0), Units::Px(50.0))
            })
            .add_child(|item| {
                item.set_background_color(1.0, 0.0, 0.0, 1.0)
                    .set_size(Units::Px(150.0), Units::Px(50.0))
            })
    })
    .finish()
}
