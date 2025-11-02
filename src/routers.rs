use dioxus::prelude::*;
use crate::pages::{Home, Profile, Task};
use crate::components::TabBarWrapper;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(TabBarWrapper)]
        #[route("/")]
        Home{},
        #[route("/profile/")]
        Profile{},
    #[end_layout]
    #[route("/task/")]
    Task{},
}
