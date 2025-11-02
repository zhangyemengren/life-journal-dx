use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    rsx! {
        div {
            h1 { "User" }
        }
    }
}
