#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut local_timestamps = use_signal::<Vec<u128>>(|| vec![]);
    let mut remote_timestamps = use_signal::<Vec<u128>>(|| vec![]);

    rsx! {
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    loop {
                        let t: u128 = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        local_timestamps.push(t);
                        let mut eval = eval(
                            &format!(
                                r#"
                    let t = {t};
                    console.log("local timestamp",t);
                    dioxus.send(t);
                    "#,
                            ),
                        );
                        let received = eval.recv().await;
                        let Ok(received) = received else {
                            println!("Failure to recv! {:?}", received);
                            return;
                        };
                        let parsed = serde_json::from_value::<u128>(received).unwrap();
                        remote_timestamps.push(parsed);
                    }
                },
                "Send timestamps in a loop"
            }
        }

        div { {format!("local_timestamps: {:?}",local_timestamps)  } }
        div { {format!("remote_timestamps: {:?}", remote_timestamps)  } }
    }
}
