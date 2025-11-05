use dioxus::prelude::*;
use crate::routers::Route;

#[component]
pub fn TabBar() -> Element {
    rsx! {
        nav {
            class: "fixed left-0 w-full flex justify-around items-center bg-gray-700 h-16",
            style: "bottom: 0; padding-bottom: max(0.5rem, env(safe-area-inset-bottom));",
            Link {
                to: Route::Home{},
                "🏠 首页" }
            Link {
                to: Route::Profile{},
                "👤 我的"
            }
        }
    }
}

#[component]
pub fn TabBarWrapper() -> Element {
    rsx! {
        Outlet::<Route> {}
        TabBar {}
    }
}
