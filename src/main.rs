mod file;
mod util;
mod web;

use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, instrument, trace};
use tracing_subscriber::EnvFilter;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

type EResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const DEFAULT_TRACING_DIRECTIVES: &'static str = "kount=info";

#[derive(Clone, Debug, Default, Serialize)]
pub struct State {
    keyboard_presses: usize,
}

#[tokio::main]
async fn main() -> EResult<()> {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let directives = std::env::var("RUST_LOG").unwrap_or(DEFAULT_TRACING_DIRECTIVES.to_string());
    let filter = EnvFilter::new(directives);
    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("unable to set global default tracing subscriber");

    let shared_state = Arc::new(RwLock::new(State::default()));
    debug!({ state = ?shared_state }, "initialised shared state with default options");

    tokio::spawn(web::handler(shared_state.clone()));
    tokio::spawn(file::handler(shared_state.clone()));
    event_handler(shared_state).expect("an error occured in the event handler");

    Ok(())
}

#[instrument(fields(event), skip(shared_state))]
fn event_handler(shared_state: Arc<RwLock<State>>) -> EResult<()> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;
    window.set_visible(false);

    event_loop.run(move |event, _, control_flow| {
        trace!({ ?event, ?control_flow }, "handling event");
        *control_flow = ControlFlow::Wait;

        let shared_state = shared_state.clone();
        match event {
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(_),
                        ..
                    }),
                ..
            } => {
                tokio::spawn(async move {
                    let mut state = shared_state.write().await;
                    (*state).keyboard_presses += 1;
                });
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
