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
            .gap_4()
            .text_color(colors.on_neutral())
            .bg(colors.neutral())
            .children([
                Button::new(1)
                    .appearance(ButtonAppearance::Primary)
                    .child("Primary"),
                Button::new(2)
                    .appearance(ButtonAppearance::Outline)
                    .child("Outline"),
                Button::new(3)
                    .appearance(ButtonAppearance::Subtle)
                    .child("Subtle"),
                Button::new(4)
                    .appearance(ButtonAppearance::Transparent)
                    .child("Transparent"),
            ])
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
