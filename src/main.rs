mod pages;
mod routers;
mod components;

use dioxus::prelude::*;
use routers::Route;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}



#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}
