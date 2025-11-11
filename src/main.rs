use dioxus::prelude::*;
use dioxus_desktop::launch::launch;

use views::{HomePage, SimulatedAnnealingPage, RouteOutlet, ART1Page, ACOPage, BackpropPage};

mod components;
mod views;
mod utils;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]

enum Route {
    #[layout(RouteOutlet)]
        #[route("/")]
        HomePage {},
        #[route("/simulated-annealing")]
        SimulatedAnnealingPage {},
        #[route("/art1")]
        ART1Page,
        #[route("/aco")]
        ACOPage,
        #[route("/backprop")]
        BackpropPage
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_desktop::launch::launch(App, Vec::new(), Vec::new());
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
