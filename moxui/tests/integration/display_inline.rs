use crate::WgpuCtx;
use calc_units::Units;
use moxui::tree;

pub fn display_inline(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    moxui::tree::Tree::new(&wgpu_ctx.device, &wgpu_ctx.surface_config, |surface| {
        surface
            .set_background_color(0.1, 0.3, 1.0, 1.0)
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(0.0, 0.0, 1.0, 1.0)
                    .set_size(Units::Px(50.0), Units::Px(50.0))
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.0, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(50.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(0.0, 0.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(50.0), Units::Auto)
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
                    .add_child(|item| {
                        item.set_size(Units::Px(0.0), Units::Px(100.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_background_color(1.0, 0.0, 0.0, 1.0)
                    .set_size(Units::Auto, Units::Px(50.0))
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(0.0, 0.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(50.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.5, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(0.5, 0.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.5, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(0.5, 0.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.5, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(0.5, 0.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.5, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(0.5, 0.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_size(Units::Px(250.0), Units::Px(50.0))
                            .set_background_color(0.0, 0.0, 0.0, 0.0)
                    })
            })
            .add_child(|item| {
                item.set_background_color(1.0, 0.0, 0.0, 1.0)
                    .set_size(Units::Auto, Units::Px(30.0))
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.5, 1.0)
                    .set_padding(
                        Units::Px(10.0),
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(100.0),
                    )
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.5, 1.0)
                    .set_padding(
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(10.0),
                        Units::Px(100.0),
                    )
                    .set_margin(
                        Units::Px(0.0),
                        Units::Px(10.0),
                        Units::Px(0.0),
                        Units::Px(10.0),
                    )
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::Inline)
                    .set_background_color(1.0, 0.0, 0.5, 1.0)
                    .set_padding(
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(10.0),
                        Units::Px(100.0),
                    )
            })
    })
    .finish()
}
