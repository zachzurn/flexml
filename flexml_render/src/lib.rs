use std::fs::File;
use std::num::NonZeroUsize;
use std::path::PathBuf;

use anyhow::{bail, Result};
use vello::kurbo::{Affine, Rect, RoundedRect, RoundedRectRadii, Stroke};
use vello::peniko::{Color, Fill};
use vello::util::{block_on_wgpu, RenderContext};
use vello::{kurbo, Scene};
use vello::RendererOptions;

use flexml_dom::layout::fragments::{FragmentGroup, Rect as FragmentRect};

use wgpu::{
    BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, TextureDescriptor,
    TextureFormat, TextureUsages,
};
use flexml_dom::layout::{FlexmlLayout, FlexmlLayoutContext, FlexmlPage};
use flexml_dom::layout::fragments::{Fragment, FragmentKind, GlyphRunFragment};

fn kurbo_rect_from_bounds(bounds: &FragmentRect) -> kurbo::Rect {
    kurbo::Rect::new(
        bounds.x as f64,
        bounds.y as f64,
        (bounds.x + bounds.width) as f64,
        (bounds.y + bounds.height) as f64,
    )
}

fn render_fragment_group(scene: &mut Scene, context: &FlexmlLayoutContext, group: &FragmentGroup) {
    for fragment in &group.fragments {
        match &fragment.kind {
            FragmentKind::ColorBackground { color, radius } => {
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgba8(color.0, color.1, color.2, color.3),
                    None,
                    &RoundedRect::from_rect(
                        kurbo_rect_from_bounds(&fragment.bounds),
                        RoundedRectRadii::new(
                            radius.top_left as f64,
                            radius.top_right as f64,
                            radius.bottom_left as f64,
                            radius.bottom_right as f64,
                        ),
                    ),
                );
            }
            FragmentKind::ColorBorder { radius, color, weight } => {
                if *weight > 0.0 {
                    let stroke = Stroke {
                        width: *weight as f64,
                        ..Default::default()
                    };

                    scene.stroke(
                        &stroke,
                        Affine::IDENTITY,
                        Color::from_rgba8(color.0, color.1, color.2, color.3),
                        None,
                        &RoundedRect::from_rect(
                            kurbo_rect_from_bounds(&fragment.bounds),
                            RoundedRectRadii::new(
                                radius.top_left as f64,
                                radius.top_right as f64,
                                radius.bottom_left as f64,
                                radius.bottom_right as f64,
                            ),
                        ),
                    );
                }
            }
            FragmentKind::Debug => {
                let stroke = Stroke {
                    width: 1.0,
                    ..Default::default()
                };

                scene.stroke(
                    &stroke,
                    Affine::IDENTITY,
                    Color::from_rgba8(255,0,0, 50),
                    None,
                    &RoundedRect::from_rect(
                        kurbo_rect_from_bounds(&fragment.bounds),
                        RoundedRectRadii::new(
                            0.0,0.0,0.0,0.0,
                        ),
                    ),
                );
            }
            FragmentKind::Text(glyph_run) => {
                // Map Parley Glyphs to Vello Glyphs with absolute position
                let vello_glyphs: Vec<vello::Glyph> = glyph_run
                    .glyphs
                    .iter()
                    .map(|g| vello::Glyph {
                        id: g.id as u32,
                        x: g.x + fragment.bounds.x,
                        y: g.y + fragment.bounds.y,
                    })
                    .collect();

                // Use the style color (assuming style stores RGBA u8 slice)
                let rgba = glyph_run.style.brush;
                let color = vello::peniko::Color::from_rgba8(rgba[0], rgba[1], rgba[2], rgba[3]);

                // Handles faux skewing
                let glyph_xform = glyph_run.synthesis
                    .skew()
                    .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));

                let mut cursor_x = fragment.bounds.x;
                let cursor_y = fragment.bounds.y + glyph_run.baseline;

                scene
                    .draw_glyphs(&glyph_run.font)
                    .brush(color)
                    .hint(false)
                    .transform(Affine::IDENTITY)
                    .glyph_transform(glyph_xform)
                    .font_size(glyph_run.font_size)
                    .normalized_coords(glyph_run.normalized_coords.as_slice())
                    .draw(
                        Fill::NonZero,
                        glyph_run.glyphs.iter().map(|glyph| {
                            let gx = cursor_x + glyph.x;
                            let gy = cursor_y - glyph.y;
                            cursor_x += glyph.advance;

                            vello::Glyph {
                                id: glyph.id as u32,
                                x: gx,
                                y: gy,
                            }
                        }),
                    );
            }
        }
    }

    for subgroup in &group.subgroups {
        render_fragment_group(scene, context, subgroup);
    }
}


pub async fn render_layout(layout: &FlexmlLayout) -> Result<()> {
    let mut context = RenderContext::new();
    let device_id = context.device(None).await.ok_or_else(|| anyhow::anyhow!("No device"))?;
    let device_handle = &mut context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;

    let mut renderer = vello::Renderer::new(
        device,
        RendererOptions {
            use_cpu: false,
            num_init_threads: NonZeroUsize::new(1),
            antialiasing_support: vello::AaSupport::area_only(),
            ..Default::default()
        },
    )
        .or_else(|_| bail!("Failed to create renderer"))?;

    let mut scene = Scene::new();

    for group in &layout.pages[0].fragments {
        group.print_tree("FRAGMENT GROUP");
        render_fragment_group(&mut scene, &layout.context, group);
    }

    let width = layout.page_width as u32;
    let height = layout.page_height as u32;

    println!("Page w {} h {}", width, height);

    let target = device.create_texture(&TextureDescriptor {
        label: Some("Target texture"),
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&Default::default());

    // Render it
    renderer.render_to_texture(
        device,
        queue,
        &scene,
        &view,
        &vello::RenderParams {
            base_color: Color::TRANSPARENT,
            width,
            height,
            antialiasing_method: vello::AaConfig::Area,
        },
    )?;

    // Copy texture to CPU
    let padded_bytes_per_row = (width * 4 + 255) & !255;
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Output Buffer"),
        size: (padded_bytes_per_row * height) as u64,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Encoder") });
    encoder.copy_texture_to_buffer(
        target.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(Some(encoder.finish()));
    let slice = buffer.slice(..);
    let (send, recv) = futures_intrusive::channel::shared::oneshot_channel();
    slice.map_async(wgpu::MapMode::Read, move |v| send.send(v).unwrap());
    block_on_wgpu(device, recv.receive()).unwrap().unwrap();
    let data = slice.get_mapped_range();

    // Remove padding
    let mut image_data = Vec::with_capacity((width * height * 4) as usize);
    for row in 0..height {
        let start = (row * padded_bytes_per_row) as usize;
        image_data.extend_from_slice(&data[start..start + (width * 4) as usize]);
    }

    // Save as PNG
    let output_path = PathBuf::from("box.png");
    let mut file = File::create(&output_path)?;
    let mut encoder = png::Encoder::new(&mut file, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image_data)?;
    writer.finish()?;

    println!("Rendered box to {:?}", output_path);

    Ok(())
}


#[cfg(test)]
mod tests {
    use flexml_dom::document::parser::FlexmlDocument;
    use flexml_dom::layout::{FlexmlLayout, FlexmlLayoutContext};
    use super::*;

    #[test]
    fn test_render_box_scene() -> Result<()> {
        //let input = "[width: 5in + height: 2in + bgColor: #ff0000AA this is some text \r\n and some more on a new line] [box + bgColor: #00FF00AA + height: 1in]";

        let input = "[color: #ff0000 We have 😄 some [bold Bold like ] text! ]";

        let document = FlexmlDocument::new(input)
            .parse();

        let layout = FlexmlLayout::new(&document, FlexmlLayoutContext::default());

        pollster::block_on(render_layout(&layout))?;
        Ok(())
    }
}
