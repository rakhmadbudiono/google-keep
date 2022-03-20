extern crate image;
use std::path::Path;

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Icon, WindowBuilder},
    },
    webview::WebViewBuilder,
};

fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn main() -> wry::Result<()> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/static/icon.png");
    let icon = load_icon(Path::new(path));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Google Keep")
        .with_window_icon(Some(icon))
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_url("https://keep.google.com/")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
