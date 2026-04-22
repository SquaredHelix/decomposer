use crate::parse::Icon;
use resvg::tiny_skia;
use resvg::usvg;

pub fn render(icon: Icon, output: String) {
    let mut pixmap = tiny_skia::Pixmap::new(1024, 1024).unwrap();
    let opt = usvg::Options::default();
    for group in icon.root.children.into_iter().rev() {
        for layer in group.children.into_iter().rev() {
            let image_data = layer.image;
            if layer.image_type == "svg" {
                let tree = usvg::Tree::from_data(&image_data, &opt).expect("Failed to decode SVG");
                let size = tree.size();
                resvg::render(
                    &tree,
                    tiny_skia::Transform::from_scale(layer.scale as f32, layer.scale as f32)
                        .post_translate(
                            layer.position.0 as f32 + 512.0
                                - size.width() * layer.scale as f32 / 2.0,
                            layer.position.1 as f32 + 512.0
                                - size.height() * layer.scale as f32 / 2.0,
                        ),
                    &mut pixmap.as_mut(),
                );
            } else if layer.image_type == "png" {
                let png_pixmap =
                    tiny_skia::Pixmap::decode_png(&image_data).expect("Failed to decode PNG");
                let paint = tiny_skia::Paint {
                    shader: tiny_skia::Pattern::new(
                        png_pixmap.as_ref(),
                        tiny_skia::SpreadMode::Pad,
                        tiny_skia::FilterQuality::Bicubic,
                        1.0,
                        tiny_skia::Transform::from_scale(layer.scale as f32, layer.scale as f32)
                            .post_translate(
                                layer.position.0 as f32 + 512.0
                                    - png_pixmap.width() as f32 * layer.scale as f32 / 2.0,
                                layer.position.1 as f32 + 512.0
                                    - png_pixmap.height() as f32 * layer.scale as f32 / 2.0,
                            ),
                    ),
                    ..Default::default()
                };
                let rect = tiny_skia::Rect::from_xywh(0.0, 0.0, 1024.0, 1024.0).unwrap();
                pixmap.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
            }
        }
    }
    pixmap.save_png(output).expect("Failed to save image");
}
