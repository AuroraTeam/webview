#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::path::PathBuf;

use wry::WebContext;

#[napi]
pub struct Window {
  title: String,
  url: String,
  web_context_dir: String,
}

#[napi]
impl Window {
  #[napi(constructor)]
  pub fn new(web_context_dir: String) -> Self {
    Window {
      title: String::new(),
      url: String::new(),
      web_context_dir,
    }
  }

  #[napi]
  pub fn set_title(&mut self, title: String) {
    self.title = title;
  }

  #[napi]
  pub fn set_url(&mut self, url: String) {
    self.url = url;
  }

  #[napi]
  pub fn create(&self) {
    use tao::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    };
    use wry::WebViewBuilder;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title(&self.title)
      .build(&event_loop)
      .unwrap();

    let path = PathBuf::from(&self.web_context_dir);
    let mut web_context = WebContext::new(Some(path));

    let _webview = WebViewBuilder::with_web_context(&mut web_context)
      .with_url(&self.url)
      .build(&window)
      .unwrap();

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;

      match event {
        Event::NewEvents(StartCause::Init) => println!("New window process started"),
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
  }
}
