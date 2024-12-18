use crate::WgpuCtx;
use calc_units::Units;
use moxui::tree;

pub fn scroll(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    moxui::tree::Tree::new(&wgpu_ctx.device, &wgpu_ctx.surface_config, |surface| {
        surface
            .set_background_color(0.0, 0.0, 0.0, 0.0)
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::InlineBlock)
                    .set_background_color(0.0, 0.0, 1.0, 1.0)
                    .set_size(Some(Units::Perc(150.0)), Some(Units::Px(50.0)))
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
                    .set_size(Some(Units::Px(50.0)), Some(Units::Perc(150.0)))
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::InlineBlock)
                    .set_background_color(0.0, 1.0, 0.0, 1.0)
                    .set_size(Some(Units::Px(150.0)), Some(Units::Px(150.0)))
            })
            .add_child(|item| {
                item.set_position(moxui::rectangle::Position::Fixed)
                    .set_background_color(1.0, 1.0, 0.0, 1.0)
                    .set_size(Some(Units::Px(150.0)), Some(Units::Px(150.0)))
                    .set_margin(
                        Units::Px(50.0),
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(50.0),
                    )
            })
    })
    .finish()
}
