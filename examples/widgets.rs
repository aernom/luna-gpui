use gpui::*;
use luna::*;

struct AlfaRobot {
    selected_tab: Option<&'static str>,
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
            .bg(colors.surface())
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
                        .child("Subtle"),
                ]),
            )
            .child(
                h_flex().gap_2().children([
                    Button::new(4)
                        .appearance(ButtonAppearance::Primary)
                        .disabled(true)
                        .child("Primary Disabled"),
                    Button::new(5)
                        .appearance(ButtonAppearance::Outline)
                        .disabled(true)
                        .child("Outline Disabled"),
                    Button::new(6)
                        .appearance(ButtonAppearance::Subtle)
                        .disabled(true)
                        .child("Subtle Disabled"),
                ]),
            )
            .child(
                h_flex().gap_2().children([
                    Button::new(7)
                        .appearance(ButtonAppearance::Outline)
                        .shape(ButtonShape::Rounded)
                        .child("Rounded"),
                    Button::new(8)
                        .appearance(ButtonAppearance::Outline)
                        .shape(ButtonShape::Circular)
                        .child("Circular"),
                    Button::new(9)
                        .appearance(ButtonAppearance::Outline)
                        .shape(ButtonShape::Square)
                        .child("Square"),
                ]),
            )
            .child(Divider::horizontal())
            .child(Divider::horizontal().style(DividerStyle::Subtle))
            .child(Divider::horizontal().style(DividerStyle::Strong))
            .child(Divider::horizontal().style(DividerStyle::Primary))
            .child(Divider::horizontal().style(DividerStyle::Custom(rgb(0xc239b3).into())))
            .child(
                h_flex().children([
                    Tab::new("enex", "Enex", self.selected_tab == Some("enex")).on_click(
                        cx.listener(|view, _, cx| {
                            view.selected_tab = Some("enex");
                            cx.notify();
                        }),
                    ),
                    Tab::new("epta", "Epta", self.selected_tab == Some("epta")).on_click(
                        cx.listener(|view, _, cx| {
                            view.selected_tab = Some("epta");
                            cx.notify();
                        }),
                    ),
                    Tab::new(
                        "disabled",
                        "Disabled",
                        self.selected_tab == Some("disabled"),
                    )
                    .disabled(true)
                    .on_click(cx.listener(|view, _, cx| {
                        view.selected_tab = Some("epta");
                        cx.notify();
                    })),
                ]),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.set_global(Theme::system(cx));
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
            |cx| cx.new_view(|_cx| AlfaRobot { selected_tab: None }),
        )
        .unwrap();
    });
}
