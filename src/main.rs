use std::time::Duration;

use dioxus::{
    desktop::{launch::launch_virtual_dom_blocking, Config},
    prelude::*,
};
use dioxus_toast::{ToastInfo, ToastManager};
use tokio::task::{spawn_local, LocalSet};

#[derive(Clone, Copy)]
struct DioxusMainBridgeContext {
    rsx_signal: Signal<Result<VNode, RenderError>>,
}

/// Run a local task that updates an rsx signal asynchronously
async fn contentful_main(mut ctx: DioxusMainBridgeContext) {
    let mut toast = use_signal(|| ToastManager::default());

    spawn_local(async move {
        let mut i = 0;
        loop {
            let my_rsx = rsx! {
                dioxus_toast::ToastFrame { manager: toast }
                h1 { {format!("{}", i)} }
            };
            i += 1;
            ctx.rsx_signal.set(my_rsx);
            let _id = toast
                .write()
                .popup(dioxus_toast::ToastInfo::simple(i.to_string().as_str()));
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });
}

#[component]
pub fn App() -> Element {
    rsx! {
        h1 { "placeholder" }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::default();
    let virtual_dom = VirtualDom::new(|| {
        let rsx_signal = use_signal(|| {
            rsx! {
                h1 { "placeholder" }
            }
        });
        let ctx = DioxusMainBridgeContext { rsx_signal };

        // Wrap our contentful_main in a LocalSet so we can use async with !Send types
        async fn local_set_wrapper(ctx: DioxusMainBridgeContext) {
            let executor = LocalSet::new();
            executor.spawn_local(contentful_main(ctx));
            executor.await;
        }

        use_future(move || local_set_wrapper(ctx));

        rsx_signal.cloned()
    });

    launch_virtual_dom_blocking(virtual_dom, config);
}
