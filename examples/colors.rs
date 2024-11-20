use gpui::*;
use luna::*;

struct AlfaRobot {}

impl Render for AlfaRobot {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let colors = Theme::of(cx).color_scheme();

        div()
            .flex()
            .w_full()
            .h_full()
            .text_color(rgb(0xffffff))
            .children([
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Button::new("test").label("Push me!"))
                    .text_size(px(24.))
                    .bg(colors.surface_bright())
                    .text_color(colors.on_surface()),
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("Right")
                    .text_size(px(24.))
                    .bg(colors.surface())
                    .text_color(colors.on_surface()),
            ])
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.set_global(Theme::light());
        cx.activate(true);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(600.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("AlfaRobot".into()),
                    appears_transparent: true,
                    ..TitlebarOptions::default()
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| AlfaRobot {}),
        )
        .unwrap();
    });
}
