//! The typical TodoMVC app, implemented in Dioxus.

use dioxus::desktop::tao::event::{Event, WindowEvent};
use dioxus::desktop::use_wry_event_handler;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        RecursiveComponent { contents: vec!["Hello".to_string(), "World".to_string()] }
    }
}

#[component]
pub fn RecursiveComponent(contents: Vec<String>) -> Element {
    use_wry_event_handler(move |e, _| {
        if let Event::WindowEvent {
            event: WindowEvent::Moved(_),
            window_id,
            ..
        } = e
        {
            // if window_id != &window_id_signal.cloned().unwrap() {
            //     println!("matching window ids");
            //     window_signal.cloned().unwrap().close_window(window_id.clone());
            //     return;
            // }
            println!("event {:?} {:?}", e, window_id);
        }
    });

    if !contents.is_empty() {
        let first = contents.remove(0);
        let rest = contents;
        if rest.len() > 0 {
            use_after_render(move || {
                let dom = VirtualDom::new_with_props(
                    RecursiveComponent,
                    RecursiveComponentProps {
                        contents: rest.to_vec(),
                    },
                );
                dioxus::desktop::window().new_window(dom, Default::default());
            });
        }

        rsx! {
            div { {format!("{:?}",first)} }
        }
    } else {
        rsx! {
            div { "Tried to render a component with no contents" }
        }
    }
}
