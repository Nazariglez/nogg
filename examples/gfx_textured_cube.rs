use gamekit::app::event;
use gamekit::gfx::{
    BindGroup, Buffer, Color, CullMode, Gfx, IndexFormat, RenderPipeline, Renderer, UniformBinding,
    VertexFormat, VertexLayout,
};
use gamekit::math::{Mat4, Vec3};
use gamekit::platform::Platform;
use gamekit::prelude::*;
use gamekit::time::Time;
use gk_gfx::TextureBinding;

// language=wgsl
const SHADER: &str = r#"
struct Transform {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> transform: Transform;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.position = transform.mvp * model.position;
    return out;
}

@group(0) @binding(1)
var t_texture: texture_2d<f32>;
@group(0) @binding(2)
var s_texture: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.tex_coords);;
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    vbo: Buffer,
    ubo: Buffer,
    bind_group: BindGroup,
    angle: f32,
    mvp: Mat4,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            -1.0,-1.0,-1.0,     0.000059,0.000004,
            -1.0,-1.0,1.0,      0.000103,0.336048,
            -1.0,1.0,1.0,       0.335973,0.335903,
            1.0,1.0,-1.0,       1.000023,0.000013,
            -1.0,-1.0,-1.0,     0.667979,0.335851,
            -1.0,1.0,-1.0,      0.999958,0.336064,
            1.0,-1.0,1.0,       0.667979,0.335851,
            -1.0,-1.0,-1.0,     0.336024,0.671877,
            1.0,-1.0,-1.0,      0.667969,0.671889,
            1.0,1.0,-1.0,       1.000023,0.000013,
            1.0,-1.0,-1.0,      0.668104,0.000013,
            -1.0,-1.0,-1.0,     0.667979,0.335851,
            -1.0,-1.0,-1.0,     0.000059,0.000004,
            -1.0,1.0,1.0,       0.335973,0.335903,
            -1.0,1.0,-1.0,      0.336098,0.000071,
            1.0,-1.0,1.0,       0.667979,0.335851,
            -1.0,-1.0,1.0,      0.335973,0.335903,
            -1.0,-1.0,-1.0,     0.336024,0.671877,
            -1.0,1.0,1.0,       1.000004,0.671847,
            -1.0,-1.0,1.0,      0.999958,0.336064,
            1.0,-1.0,1.0,       0.667979,0.335851,
            1.0,1.0,1.0,        0.668104,0.000013,
            1.0,-1.0,-1.0,      0.335973,0.335903,
            1.0,1.0,-1.0,       0.667979,0.335851,
            1.0,-1.0,-1.0,      0.335973,0.335903,
            1.0,1.0,1.0,        0.668104,0.000013,
            1.0,-1.0,1.0,       0.336098,0.000071,
            1.0,1.0,1.0,        0.000103,0.336048,
            1.0,1.0,-1.0,       0.000004,0.671870,
            -1.0,1.0,-1.0,      0.336024,0.671877,
            1.0,1.0,1.0,        0.000103,0.336048,
            -1.0,1.0,-1.0,      0.336024,0.671877,
            -1.0,1.0,1.0,       0.335973,0.335903,
            1.0,1.0,1.0,        0.667969,0.671889,
            -1.0,1.0,1.0,       1.000004,0.671847,
            1.0,-1.0,1.0,       0.667979,0.335_851
        ];
        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        let mvp = create_mvp();
        let ubo = gfx
            .create_uniform_buffer(mvp.as_ref())
            .with_write_flag(true)
            .build()?;

        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/cube.png"))
            .build()?;

        let sampler = gfx.create_sampler().build()?;

        let bind_group = gfx
            .create_bind_group()
            .with_uniform(UniformBinding::new(0, &ubo).with_vertex_visibility(true))
            .with_texture(
                TextureBinding::new()
                    .with_texture(1, &texture)
                    .with_sampler(2, &sampler)
                    .with_fragment_visibility(true),
            )
            .build()?;

        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x3)
                    .with_attr(1, VertexFormat::Float32x2),
            )
            .with_bind_group(&bind_group)
            .with_index_format(IndexFormat::UInt16)
            .with_cull_mode(CullMode::Back)
            .build()?;

        Ok(State {
            pip,
            vbo,
            ubo,
            bind_group,
            angle: 0.0,
            mvp,
        })
    }

    fn rotated_mvp(&self) -> Mat4 {
        self.mvp * Mat4::from_rotation_x(self.angle) * Mat4::from_rotation_y(self.angle)
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(Platform::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .on(on_update)
        .build()
}

fn on_update(_: &event::Update, time: &mut Time, state: &mut State) {
    state.angle += 0.6 * time.delta_f32();
}

fn on_draw(evt: &event::DrawRequest, gfx: &mut Gfx, state: &mut State) {
    // update mvp
    gfx.write_buffer(&state.ubo)
        .with_data(state.rotated_mvp().as_ref())
        .build()
        .unwrap();

    let mut renderer = Renderer::new();
    renderer.begin(1600, 1200);
    renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, None);
    renderer.apply_pipeline(&state.pip);
    renderer.apply_buffers(&[&state.vbo]);
    renderer.apply_bindings(&[&state.bind_group]);
    renderer.draw(0..36);
    gfx.render(evt.window_id, &renderer).unwrap();
}

fn create_mvp() -> Mat4 {
    let projection = Mat4::perspective_rh_gl(45.0, 4.0 / 3.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(
        Vec3::new(4.0, 3.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    Mat4::IDENTITY * projection * view
}
