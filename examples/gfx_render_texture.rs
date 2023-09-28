use gamekit::app::App;
use gamekit::gfx::{
    BindGroup, BindGroupLayout, BindingType, BlendMode, Buffer, Color, GKRenderPipeline,
    GKRenderTexture, GKTexture, Gfx, IndexFormat, RenderPipeline, RenderTexture, Renderer,
    VertexFormat, VertexLayout,
};
use gamekit::prelude::*;
use gamekit::sys::event::DrawEvent;
use gamekit::time::Time;
use std::ops::Range;

// language=wgsl
const SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position.x, model.position.y * -1.0, 0.0, 1.0);
    return out;
}

@group(0) @binding(0)
var t_texture: texture_2d<f32>;
@group(0) @binding(1)
var s_texture: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.tex_coords);
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    vbo: Buffer,
    ebo: Buffer,
    texture_bind_group: BindGroup,
    rt: RenderTexture,
    rt_bind_group: BindGroup,
    rt2: RenderTexture,
    rt2_bind_group: BindGroup,
    texture_initiated: bool,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_label("Image Pipeline")
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x2)
                    .with_attr(1, VertexFormat::Float32x2),
            )
            .with_bind_group_layout(
                BindGroupLayout::new()
                    .with_entry(BindingType::texture(0).with_fragment_visibility(true))
                    .with_entry(BindingType::sampler(1).with_fragment_visibility(true)),
            )
            .with_index_format(IndexFormat::UInt16)
            .with_blend_mode(BlendMode::NORMAL)
            .build()?;

        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/ferris.png"))
            .build()?;

        let sampler = gfx.create_sampler().build()?;

        let texture_bind_group = gfx
            .create_bind_group()
            .with_layout(pip.bind_group_layout_id(0)?)
            .with_texture(0, &texture)
            .with_sampler(1, &sampler)
            .build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            //pos               //coords
            0.9,  0.9,     1.0, 1.0,
            0.9, -0.9,     1.0, 0.0,
            -0.9, -0.9,    0.0, 0.0,
            -0.9,  0.9,    0.0, 1.0,

            //pos               //coords
            0.8,  0.8,     1.0, 1.0,
            0.8, -0.8,     1.0, 0.0,
            -0.8, -0.8,    0.0, 0.0,
            -0.8,  0.8,    0.0, 1.0
        ];
        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 3,
            1, 2, 3,

            4, 5, 7,
            5, 6, 7,
        ];
        let ebo = gfx.create_index_buffer(indices).build()?;

        let rt = gfx
            .create_render_texture()
            .with_size(texture.width(), texture.height())
            .build()?;

        let rt_bind_group = gfx
            .create_bind_group()
            .with_layout(pip.bind_group_layout_id(0)?)
            .with_texture(0, rt.texture())
            .with_sampler(1, &sampler)
            .build()?;

        let rt2 = gfx
            .create_render_texture()
            .with_size(texture.width(), texture.height())
            .build()?;

        let rt2_bind_group = gfx
            .create_bind_group()
            .with_layout(pip.bind_group_layout_id(0)?)
            .with_texture(0, rt2.texture())
            .with_sampler(1, &sampler)
            .build()?;

        Ok(State {
            pip,
            vbo,
            ebo,
            texture_bind_group,
            rt,
            rt_bind_group,
            rt2,
            rt2_bind_group,
            texture_initiated: false,
        })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &DrawEvent, gfx: &mut Gfx, state: &mut State) {
    let frame = gfx.create_frame(evt.window_id).unwrap();

    if !state.texture_initiated {
        for i in 0..30 {
            // the first pass will draw the texture to the rt1
            let tex = if i == 0 {
                &state.texture_bind_group
            } else {
                &state.rt_bind_group
            };

            {
                // draw rt1 to rt2
                let renderer = render_texture(state, tex, 0..6, None);
                gfx.render(&state.rt2, &renderer);

                // draw rt2 to rt1
                let renderer = render_texture(state, &state.rt2_bind_group, 0..6, None);
                gfx.render(&state.rt, &renderer);
            }

            // swap render textures
            std::mem::swap(&mut state.rt, &mut state.rt2);
            std::mem::swap(&mut state.rt_bind_group, &mut state.rt2_bind_group);
        }

        // avoid to do this on each frame
        state.texture_initiated = true;
    }

    // draw end result to the frame
    let renderer = render_texture(
        state,
        &state.rt_bind_group,
        0..6,
        Some(Color::rgb(0.1, 0.2, 0.3)),
    );
    gfx.render(&frame, &renderer).unwrap();

    // present the frame to the screen
    gfx.present(frame).unwrap();
}

fn render_texture<'a>(
    state: &'a State,
    bg: &'a BindGroup,
    range: Range<u32>,
    clear_color: Option<Color>,
) -> Renderer<'a> {
    let mut renderer = Renderer::new();
    let rpass = renderer.begin_pass();

    if let Some(color) = clear_color {
        rpass.clear_color(color);
    }

    rpass
        .pipeline(&state.pip)
        .buffers(&[&state.vbo, &state.ebo])
        .bindings(&[bg])
        .draw(range);

    renderer
}
