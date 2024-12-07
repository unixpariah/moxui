mod setup;

use moxui::rectangle::units;
use setup::WgpuCtx;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app)
}

#[derive(Default)]
pub struct App<'window> {
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'window>>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("wgpu winit example");
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.window = Some(window.clone());
            let mut wgpu_ctx = WgpuCtx::new(window.clone());

            let tree =
                moxui::tree::Tree::new(&wgpu_ctx.device, &wgpu_ctx.surface_config, |surface| {
                    surface
                        .set_background_color(0.0, 0.0, 0.0, 0.0)
                        .add_child(|item| {
                            item.set_background_color(0.0, 0.0, 1.0, 1.0)
                                .set_size(None, Some(units::Units::Px(100.0)))
                                .add_child(|item| {
                                    item.set_background_color(0.0, 1.0, 0.0, 1.0)
                                        .set_margin(
                                            units::Units::Calc(Box::new(units::CalcExpr::Sub(
                                                Box::new(units::CalcExpr::Value(
                                                    units::Units::Perc(50.0),
                                                )),
                                                Box::new(units::CalcExpr::Value(units::Units::Px(
                                                    25.0,
                                                ))),
                                            ))),
                                            units::Units::Px(0.0),
                                            units::Units::Px(0.0),
                                            units::Units::Perc(25.0),
                                        )
                                        .set_size(
                                            Some(units::Units::Perc(50.0)),
                                            Some(units::Units::Px(50.0)),
                                        )
                                })
                        })
                        .add_child(|item| {
                            item.set_background_color(1.0, 0.0, 0.0, 1.0)
                                .set_border_size(5.0, 5.0, 5.0, 5.0)
                                .set_border_color(1.0, 1.0, 1.0, 1.0)
                                .set_size(None, Some(units::Units::Px(100.0)))
                        })
                });
            wgpu_ctx.tree = Some(tree);

            self.wgpu_ctx = Some(wgpu_ctx);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(wgpu_ctx) = &self.wgpu_ctx {
                    wgpu_ctx.draw();
                }
            }
            _ => (),
        }
    }
}
