use whiskers::prelude::*;

#[sketch_app]
struct HelloWorldSketch {
    width: f64,
    height: f64,
}

impl Default for HelloWorldSketch {
    fn default() -> Self {
        Self {
            width: 400.0,
            height: 400.0,
        }
    }
}

impl App for HelloWorldSketch {
    fn update(&mut self, sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        sketch.color(Color::GREEN).stroke_width(1.0);

        sketch
            .translate(sketch.width() / 2.0, sketch.height() / 2.0)
            .polyline(
                vec![
                    (0.0, 0.0),
                    (100.0, 0.0),
                    (100.0, 100.0),
                    (20.0, 30.0),
                    (0.0, 100.0),
                ],
                true,
            );

        Ok(())
    }
}

fn main() -> Result {
    HelloWorldSketch::runner()
        .with_page_size_options(PageSize::A5H)
        .run()
}
