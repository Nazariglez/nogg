use gamekit::m2d::*;
use notan::draw::*;
use notan::math::*;
use notan::prelude::*;

const WORK_SIZE: Vec2 = Vec2::new(300.0, 300.0);

#[derive(AppState, Default)]
struct State {
    camera: Camera,
    entity: Vec2,
}

impl State {
    fn new() -> Self {
        let mut camera = Camera::new(vec2(800.0, 600.0));
        camera.set_mode(CameraMode::Fill(WORK_SIZE));
        Self {
            camera,
            entity: vec2(400.0, 300.0),
        }
    }
}

fn main() {
    let win_conf = WindowConfig::default().resizable(true);
    notan::init_with(State::new)
        .add_config(win_conf)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
        .unwrap();
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Q) {
        app.exit();
        return;
    }

    if app.keyboard.is_down(KeyCode::J) {
        let zoom = state.camera.zoom();
        state.camera.set_zoom(zoom - 10.0 * app.timer.delta_f32());
    } else if app.keyboard.is_down(KeyCode::K) {
        let zoom = state.camera.zoom();
        state.camera.set_zoom(zoom + 10.0 * app.timer.delta_f32());
    }

    if app.keyboard.is_down(KeyCode::H) {
        let rotation = state.camera.rotation();
        state
            .camera
            .set_rotation(rotation - 10f32.to_radians() * app.timer.delta_f32());
    } else if app.keyboard.is_down(KeyCode::L) {
        let rotation = state.camera.rotation();
        state
            .camera
            .set_rotation(rotation + 10f32.to_radians() * app.timer.delta_f32());
    }

    let speed = 50.0;
    if app.keyboard.is_down(KeyCode::A) {
        state.entity.x -= speed * app.timer.delta_f32();
    } else if app.keyboard.is_down(KeyCode::D) {
        state.entity.x += speed * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::W) {
        state.entity.y -= speed * app.timer.delta_f32();
    } else if app.keyboard.is_down(KeyCode::S) {
        state.entity.y += speed * app.timer.delta_f32();
    }

    let (w, h) = app.window().size();
    state.camera.set_size(w as _, h as _);
    state.camera.set_position(state.entity.x, state.entity.y);
    // dbg!(state.entity);
    state.camera.update();

    let projection = state.camera.projection();
    let transform = state.camera.transform();

    let mut draw = gfx.create_draw();
    draw.set_projection(Some(projection));
    draw.transform().push(transform);
    draw.clear(Color::BLACK);

    let start: (f32, f32) = ((WORK_SIZE * 0.5 * -1.0) + state.entity).into();
    draw.rect(start, WORK_SIZE.into())
        .stroke_color(Color::ORANGE)
        .stroke(4.0);

    draw.circle(20.0)
        .position(state.entity.x, state.entity.y)
        .fill_color(Color::RED)
        .fill()
        .stroke_color(Color::WHITE)
        .stroke(4.0);

    draw.rect((400.0, 300.0), (20.0, 20.0))
        .color(Color::MAGENTA);

    draw.line((0.0, 0.0), (800.0, 600.0)).width(10.0);

    draw.line((0.0, 600.0), (800.0, 0.0)).width(10.0);

    gfx.render(&draw);
}
