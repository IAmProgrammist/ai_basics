use dioxus::prelude::*;

use views::{HomePage, SimulatedAnnealingPage, RouteOutlet};

mod components;
mod views;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]

enum Route {
    #[layout(RouteOutlet)]
        #[route("/")]
        HomePage {},
        #[route("/simulated-annealing")]
        SimulatedAnnealingPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
