use gpui::*;
use luna::*;

struct AlfaRobot {}

impl Render for AlfaRobot {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(TitleBar::new().child(div().text_color(rgb(0xff0000)).child("Titlebar")))
            .text_color(rgb(0xffffff))
            .child("Test")
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
