use dioxus::prelude::*;

use crate::utils::FIELD_SIZE;

#[derive(Props, PartialEq, Clone)]
pub struct FieldProps {
    field: [[bool; FIELD_SIZE]; FIELD_SIZE]
}

#[component]
pub fn Field(props: FieldProps) -> Element {    
    let columns_amount = match props.field.get(0) {
        Some(field_row) => field_row.len(),
        None => 0,
    };

    rsx! {
        div {
            class: "grid grid-cols-20 grid-rows-20 gap-1",
            for i in (0..props.field.len()) {
                for j in (0..columns_amount) {
                    div { 
                        class: "rounded-md border-solid border border-gray-300 min-h-6",
                        if *props.field.get(i as usize).unwrap().get(j as usize).unwrap() { "👑" } else { "" }
                    }
                }
            }
        }
    }
}
