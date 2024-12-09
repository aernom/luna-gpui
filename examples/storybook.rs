mod pages;

use gpui::*;
use luna::*;
use pages::*;

#[derive(Debug, PartialEq, Eq)]
enum StorybookPage {
    Buttons,
    Dividers,
}

impl Into<ElementId> for StorybookPage {
    fn into(self) -> ElementId {
        match self {
            StorybookPage::Buttons => "buttons_page",
            StorybookPage::Dividers => "dividers_page",
        }
        .into()
    }
}

struct AlfaRobot {
    selected_tab: StorybookPage,
}

impl Render for AlfaRobot {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let colors = cx.theme().color_scheme();

        v_flex()
            .w_full()
            .h_full()
            .text_color(colors.on_neutral())
            .bg(colors.surface())
            .child(TitleBar::new().child(div().text_sm().child("This is a custom TitleBar")))
            .child(
                h_flex().mb_6().children([
                    Tab::new(
                        StorybookPage::Buttons,
                        "Buttons",
                        self.selected_tab == StorybookPage::Buttons,
                    )
                    .on_click(cx.listener(|view, _, cx| {
                        view.selected_tab = StorybookPage::Buttons;
                        cx.notify();
                    })),
                    Tab::new(
                        StorybookPage::Dividers,
                        "Dividers",
                        self.selected_tab == StorybookPage::Dividers,
                    )
                    .on_click(cx.listener(|view, _, cx| {
                        view.selected_tab = StorybookPage::Dividers;
                        cx.notify();
                    })),
                    Tab::new("disabled", "Disabled", false).disabled(true),
                ]),
            )
            .child(match self.selected_tab {
                StorybookPage::Buttons => buttons_page(),
                StorybookPage::Dividers => dividers_page(),
            })
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
                    selected_tab: StorybookPage::Buttons,
                })
            },
        )
        .unwrap();
    });
}
