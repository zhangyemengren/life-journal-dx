use dioxus::prelude::*;
use crate::routers::Route;

#[component]
pub fn TabBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        nav {
            class: "tab-bar",
            Link { to: Route::Home{}, "🏠 首页" }
            Link { to: Route::Profile{}, "👤 我的" }
        }
    }
}

#[component]
pub fn TabBarWrapper() -> Element {
    rsx! {
        TabBar {}
        Outlet::<Route> {}
    }
}
