use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ClusterProps {
    cluster: Vec<u64>,
    dimension: usize
}

#[component]
pub fn Cluster(props: ClusterProps) -> Element {    
    rsx! {
        div {
            class: "flex flex-col gap-4",
            h3 {
                class: "text-gray-900 dark:text-white text-md font-semibold",
                "Вектор-прототип"
            }
            ClusterElement {
                cluster_element: props.cluster[0],
                dimension: props.dimension
            }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ClusterElementProps {
    cluster_element: u64,
    dimension: usize
}

fn ClusterElement(props: ClusterElementProps) -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-between gap-1",
            for iter in 0..props.dimension {
                ClusterBlock {
                    enabled: ((props.cluster_element >> (props.dimension - iter - 1)) & 1) == 1
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ClusterBlockProps {
    enabled: bool
}

fn ClusterBlock(props: ClusterBlockProps) -> Element {
    let bg_color = if props.enabled {"bg-green-500"} else {"bg-red-500"};
    let block_class = format!("border border-white dark:border-black border-solid w-5 h-5 rounded-sm {bg_color}");

    rsx!(
        div {
            class: block_class
        }   
    )
}