mod pages;

use gpui::*;
use luna::*;
use pages::*;

struct AlfaRobot {
    selected_tab: &'static str,
}

impl Render for AlfaRobot {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let colors = cx.theme().color_scheme();

        v_flex()
            .w_full()
            .h_full()
            .text_color(colors.on_neutral())
            .bg(colors.surface())
            .child(TitleBar::new())
            .child(
                h_flex().children([
                    Tab::new(
                        "buttons_page",
                        "Buttons",
                        self.selected_tab == "buttons_page",
                    )
                    .on_click(cx.listener(|view, _, cx| {
                        view.selected_tab = "buttons_page";
                        cx.notify();
                    })),
                    Tab::new(
                        "dividers_page",
                        "Dividers",
                        self.selected_tab == "dividers_page",
                    )
                    .on_click(cx.listener(|view, _, cx| {
                        view.selected_tab = "dividers_page";
                        cx.notify();
                    })),
                    Tab::new("disabled", "Disabled", false).disabled(true),
                ]),
            )
            .child(cx.new_view(|_| ButtonsPage {}))
            .child(cx.new_view(|_| DividersPage {}))
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
            |cx| {
                cx.new_view(|_cx| AlfaRobot {
                    selected_tab: "buttons_page",
                })
            },
        )
        .unwrap();
    });
}
