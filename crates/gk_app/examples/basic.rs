use gk_app::event::AppEvent;
use gk_app::prelude::*;
use gk_core::events::{SuperEvent, SuperEvent2};
use gk_core::window::{GKWindowId, GKWindowManager};
use gk_platform::{PlatformConfig, Windows};
use gk_win::{WindowManager, WindowsConfig};
use gk_winit::{runner, Manager, Window, WinitConfig};

// pub type Windows = WindowManager<gk_winit::Window, gk_winit::Manager>;

#[derive(AppState)]
struct State {
    id: i32,
    i: i32,
    // win_id: GKWindowId,
}

struct PP {
    id: i32,
}

impl Plugin for PP {}

fn main() {
    let win_config = PlatformConfig;

    AppBuilder::init_with(|pp: &mut PP, windows: &mut Windows| {
        // let win_id = manager.create()?;
        Ok(State {
            id: 9999,
            i: pp.id,
            // win_id,
        })
    })
    .add_config(win_config)
    .unwrap()
    .add_plugin(PP { id: 1234 })
    .on_update(|state: &mut State, pp: &mut PP| {
        println!("state.id: {}x{}, pp.id: {}", state.id, state.i, pp.id);
    })
    .listen_event(|evt: &AppEvent, ee: &mut EventQueue<State>| {
        println!("-> {evt:?}");
        if let AppEvent::PostUpdate = evt {
            println!("here... {:?}", evt);
            // panic!();
        } else {
            ee.queue(SuperEvent);
        }
    })
    .listen_event(|evt: &SuperEvent, ee: &mut EventQueue<State>| {
        println!("SuperEvent");
        ee.queue(SuperEvent2);
    })
    .listen_event(
        |evt: &SuperEvent2, windows: &mut Windows, ee: &mut EventQueue<State>| {
            println!("SuperEvent2");
            windows.exit();
        },
    )
    .build()
    .unwrap();
}
