use gpui::*;
use luna::*;

struct AlfaRobot {
    selected: bool,
}

impl Render for AlfaRobot {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let colors = cx.theme().color_scheme();

        v_flex()
            .h_full()
            .w_full()
            .justify_center()
            .content_center()
            .gap_4()
            .text_color(colors.on_neutral())
            .bg(colors.neutral())
            .child(div().absolute().inset_0().child(TitleBar::new()))
            .child(
                h_flex().gap_2().children([
                    Button::new(1)
                        .appearance(ButtonAppearance::Primary)
                        .child("Primary"),
                    Button::new(2)
                        .appearance(ButtonAppearance::Outline)
                        .child("Outline"),
                    Button::new(3)
                        .appearance(ButtonAppearance::Subtle)
                        .child("Subtle")
                        .on_click(cx.listener(|view, _, cx| {
                            view.selected = !view.selected;
                            cx.notify();
                        })),
                ]),
            )
            .child(Divider::horizontal())
            .child(
                h_flex().child(Tab::new("Prova", "Tab 1").selected(self.selected).on_click(
                    cx.listener(|view, _, cx| {
                        view.selected = !view.selected;
                        cx.notify();
                    }),
                )),
            )
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
            |cx| cx.new_view(|_cx| AlfaRobot { selected: false }),
        )
        .unwrap();
    });
}
