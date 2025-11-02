use dioxus::prelude::*;

#[component]
pub fn Task() -> Element {
    rsx! {
        div {
            h1 { "任务" }
        }
    }
}
