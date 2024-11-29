use gpui::*;
use luna::*;

struct AlfaRobot {}

impl Render for AlfaRobot {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let colors = Theme::of(cx).color_scheme();

        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(colors.on_neutral())
            .bg(colors.neutral())
            .child("Test")
            .hover(|style| style.bg(colors.primary()).text_color(rgb(0x000000)))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.set_global(Theme::dark());
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
