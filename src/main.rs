#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_tui::launch(app);
}

#[derive(Props)]
struct PanelProps<'a> {
    width: usize,
    height: usize,
    name: &'a str,
    element: Element<'a>,
}

fn Panel<'a>(cx: Scope<'a, PanelProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
                border_width: "1px",
                width: "{cx.props.width}%",
                height: "{cx.props.height}%",
                justify_content: "center",
                align_items: "center",
                border_style: "inset",
                border_width: "thick",
                border_radius: "1px",
                border_color: "grey",
                if cx.props.element.is_some() {
                    rsx!(&cx.props.element)
                } else {
                    rsx!("{cx.props.name}")
                }
            }
    })
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            width: "100%",
            height: "100%",
            flex_direction: "column",
            div {
                width: "100%",
                height: "30%",
                flex_direction: "row",

                Panel {
                    height: 100,
                    width: 50,
                    name: "Metadata",
                    element: None,
                }

                Panel {
                    height: 100,
                    width: 50,
                    name: "Transfer Panel"
                    element: None,
                }
            }

            Panel {
                height: 10,
                width: 100,
                name: "Completion"
                element: None,
            }

            div {
                width: "100%",
                height: "60%",
                flex_direction: "row",

                Panel {
                    height: 100,
                    width: 40,
                    name: "Files",
                    element: None,
                }

                Panel {
                    height: 100,
                    width: 20,
                    name: "Pieces",
                    element: None,
                }

                Panel {
                    height: 100,
                    width: 40,
                    name: "Peers",
                    element: None,
                }
            }
        }
    })
}
