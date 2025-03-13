#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::path::PathBuf;

use wry::{webview_version, WebContext, WebView};

use tao::{
  dpi::LogicalSize,
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window as TaoWindow, WindowBuilder},
};
use wry::WebViewBuilder;

#[napi(object)]
pub struct WindowConstructor {
  /* Test */
  pub app_name: Option<String>,
  // Pos
  pub x: Option<u16>,
  pub y: Option<u16>,
  // Size
  pub width: Option<u16>,
  pub height: Option<u16>,
  pub min_width: Option<u16>,
  pub min_height: Option<u16>,
  pub max_width: Option<u16>,
  pub max_height: Option<u16>,
  pub resizable: Option<bool>,
  // Other
  pub title: Option<String>,
  pub icon: Option<String>,
  pub show: Option<bool>,
  pub frame: Option<bool>,
}

impl Default for WindowConstructor {
  fn default() -> Self {
    WindowConstructor {
      app_name: Some("Glacier App".to_string()),
      x: None,
      y: None,
      width: Some(800),
      height: Some(600),
      min_width: None,
      min_height: None,
      max_width: None,
      max_height: None,
      resizable: Some(true),
      title: Some("Glacier App".to_string()),
      icon: None,
      show: None,
      frame: None,
    }
  }
}

#[napi]
pub struct Window {
  options: WindowConstructor,

  url: Option<String>,
  html: Option<String>,

  window: Option<TaoWindow>,
  webwiew: Option<WebView>,
}

#[napi]
impl Window {
  #[napi(constructor)]
  pub fn new(options: Option<WindowConstructor>) -> Self {
    Window {
      options: options.unwrap_or_default(),
      url: None,
      html: None,
      window: None,
      webwiew: None,
    }
  }

  #[napi]
  pub fn set_title(&mut self, title: String) {
    self.options.title = Some(title);
  }

  #[napi]
  pub fn load_url(&mut self, url: String) {
    match &self.webwiew {
      None => self.url = Some(url.clone()),
      Some(ww) => ww.load_url(url.as_str()).unwrap(),
    }
  }

  #[napi]
  pub fn load_html(&mut self, url: String) {
    match &self.webwiew {
      None => self.html = Some(url.clone()),
      Some(ww) => ww.load_html(url.as_str()).unwrap(),
    }
  }

  #[napi]
  pub fn create(&mut self) {
    let event_loop = EventLoop::new();
    self.window = Some(
      WindowBuilder::new()
        .with_title(self.get_title())
        .with_inner_size(LogicalSize::new(self.get_width(), self.get_height()))
        .build(&event_loop)
        .unwrap(),
    );

    let path = PathBuf::from(&self.get_web_context_dir());
    let mut web_context = WebContext::new(Some(path));

    let mut webview = WebViewBuilder::with_web_context(&mut web_context);
    if let Some(url) = &self.url {
      webview = webview.with_url(url);
    } else if let Some(html) = &self.html {
      webview = webview.with_html(html);
    }
    self.webwiew = Some(webview.build(self.window.as_ref().unwrap()).unwrap());

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

  #[napi]
  pub fn get_webview_version() -> String {
    webview_version().unwrap()
  }

  fn get_app_name(&self) -> String {
    self.options.app_name.clone().unwrap_or_default()
  }

  fn get_width(&self) -> u16 {
    self.options.width.unwrap_or_default()
  }

  fn get_height(&self) -> u16 {
    self.options.height.unwrap_or_default()
  }

  fn get_title(&self) -> String {
    self.options.title.clone().unwrap_or_default()
  }

  fn get_web_context_dir(&self) -> String {
    return std::env::temp_dir().to_str().unwrap().to_string() + self.get_app_name().as_str();
  }
}
