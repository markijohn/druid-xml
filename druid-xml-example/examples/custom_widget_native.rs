#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, LocalizedString, WindowDesc, Data, Lens};

pub fn main() {
    druid_xml!(
        r#"
        <flex direction="column" fn="build_main" lens="()">
            <demo_custom_widget style="border:3px solid yellow; width:200px; height:300px" />
        </flex>
        "#
    );
    
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("basic-demo").with_placeholder("Basic Demo"),
        );
    AppLauncher::with_window(window)
        .launch( () )
        .expect("launch failed");
}


fn demo_custom_widget() -> Box<dyn druid::Widget<()>> {
    use druid::{EventCtx, Widget, LifeCycleCtx, Event, Env, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, Color, kurbo::BezPath, RenderContext, Point, Rect, TextLayout, FontDescriptor, FontFamily, Affine, piet::{InterpolationMode, ImageFormat}, LifeCycle};
    use druid::piet::{Text,TextLayoutBuilder};

    struct CustomWidget;

    // If this widget has any child widgets it should call its event, update and layout
    // (and lifecycle) methods as well to make sure it works. Some things can be filtered,
    // but a general rule is to just pass it through unless you really know you don't want it.
    impl Widget<()> for CustomWidget {
        fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {}

        fn lifecycle(
            &mut self,
            _ctx: &mut LifeCycleCtx,
            _event: &LifeCycle,
            _data: &(),
            _env: &Env,
        ) {
        }

        fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}

        fn layout(
            &mut self,
            _layout_ctx: &mut LayoutCtx,
            bc: &BoxConstraints,
            _data: &(),
            _env: &Env,
        ) -> Size {
            // BoxConstraints are passed by the parent widget.
            // This method can return any Size within those constraints:
            // bc.constrain(my_size)
            //
            // To check if a dimension is infinite or not (e.g. scrolling):
            // bc.is_width_bounded() / bc.is_height_bounded()
            //
            // bx.max() returns the maximum size of the widget. Be careful
            // using this, since always make sure the widget is bounded.
            // If bx.max() is used in a scrolling widget things will probably
            // not work correctly.
            if bc.is_width_bounded() && bc.is_height_bounded() {
                bc.max()
            } else {
                let size = Size::new(100.0, 100.0);
                bc.constrain(size)
            }
        }

        // The paint method gets called last, after an event flow.
        // It goes event -> update -> layout -> paint, and each method can influence the next.
        // Basically, anything that changes the appearance of a widget causes a paint.
        fn paint(&mut self, ctx: &mut PaintCtx, data: &(), env: &Env) {
            // Clear the whole widget with the color of your choice
            // (ctx.size() returns the size of the layout rect we're painting in)
            // Note: ctx also has a `clear` method, but that clears the whole context,
            // and we only want to clear this widget's area.
            let data = "Druid + PIet + XML";
            let size = ctx.size();
            let rect = size.to_rect();
            ctx.fill(rect, &Color::WHITE);

            // We can paint with a Z index, this indicates that this code will be run
            // after the rest of the painting. Painting with z-index is done in order,
            // so first everything with z-index 1 is painted and then with z-index 2 etc.
            // As you can see this(red) curve is drawn on top of the green curve
            ctx.paint_with_z_index(1, move |ctx| {
                let mut path = BezPath::new();
                path.move_to((0.0, size.height));
                path.quad_to((40.0, 50.0), (size.width, 0.0));
                // Create a color
                let stroke_color = Color::rgb8(128, 0, 0);
                // Stroke the path with thickness 1.0
                ctx.stroke(path, &stroke_color, 5.0);
            });

            // Create an arbitrary bezier path
            let mut path = BezPath::new();
            path.move_to(Point::ORIGIN);
            path.quad_to((40.0, 50.0), (size.width, size.height));
            // Create a color
            let stroke_color = Color::rgb8(0, 128, 0);
            // Stroke the path with thickness 5.0
            ctx.stroke(path, &stroke_color, 5.0);

            // Rectangles: the path for practical people
            let rect = Rect::from_origin_size((10.0, 10.0), (100.0, 100.0));
            // Note the Color:rgba8 which includes an alpha channel (7F in this case)
            let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
            ctx.fill(rect, &fill_color);

            // Text is easy; in real use TextLayout should either be stored in the
            // widget and reused, or a label child widget to manage it all.
            // This is one way of doing it, you can also use a builder-style way.
            let mut layout = TextLayout::<String>::from_text(data);
            layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
            layout.set_text_color(fill_color);
            layout.rebuild_if_needed(ctx.text(), env);

            // Let's rotate our text slightly. First we save our current (default) context:
            ctx.with_save(|ctx| {
                // Now we can rotate the context (or set a clip path, for instance):
                // This makes it so that anything drawn after this (in the closure) is
                // transformed.
                // The transformation is in radians, but be aware it transforms the canvas,
                // not just the part you are drawing. So we draw at (80.0, 40.0) on the rotated
                // canvas, this is NOT the same position as (80.0, 40.0) on the original canvas.
                ctx.transform(Affine::rotate(std::f64::consts::FRAC_PI_4));
                layout.draw(ctx, (80.0, 40.0));
            });
            // When we exit with_save, the original context's rotation is restored

            // This is the builder-style way of drawing text.
            let text = ctx.text();
            let layout = text
                .new_text_layout(data)
                .font(FontFamily::SERIF, 24.0)
                .text_color(Color::rgb8(128, 0, 0))
                .build()
                .unwrap();
            ctx.draw_text(&layout, (100.0, 25.0));

            // Let's burn some CPU to make a (partially transparent) image buffer
            let image_data = make_image_data(256, 256);
            let image = ctx
                .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
                .unwrap();
            // The image is automatically scaled to fit the rect you pass to draw_image
            ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
        }
    }

    fn make_image_data(width: usize, height: usize) -> Vec<u8> {
        let mut result = vec![0; width * height * 4];
        for y in 0..height {
            for x in 0..width {
                let ix = (y * width + x) * 4;
                result[ix] = x as u8;
                result[ix + 1] = y as u8;
                result[ix + 2] = !(x as u8);
                result[ix + 3] = 127;
            }
        }
        result
    }

    Box::new( CustomWidget{} )
}