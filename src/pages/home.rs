use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}
