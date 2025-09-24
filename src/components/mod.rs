//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.
mod logo;
pub use logo::*;

mod lab_card;
pub use lab_card::*;

mod go_home;
pub use go_home::*;

mod input;
pub use input::*;

mod button;
pub use button::*;

mod simulated_annealing;
pub use simulated_annealing::*;