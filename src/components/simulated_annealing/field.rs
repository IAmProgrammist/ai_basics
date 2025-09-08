use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FieldProps {
    field: Vec<Vec<bool>>
}

#[component]
pub fn Field(props: FieldProps) -> Element {    
    let columns_amount = match props.field.get(0) {
        Some(field_row) => field_row.len(),
        None => 0,
    };

    rsx! {
        div {
            class: "grid grid-cols-{columns_amount} grid-rows-{props.field.len()} gap-1",
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
