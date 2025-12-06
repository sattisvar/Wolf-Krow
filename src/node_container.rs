use leptos::*;
use leptos::prelude::*;

use reactive_stores::{AtKeyed, Store};
use web_sys::MouseEvent;

#[derive(Clone, Debug)]
pub struct InputSlot {
    pub id: usize,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct OutputSlot {
    pub id: usize,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub label: String,
    pub width: f64,
    pub input_slot: Vec<InputSlot>,
    pub output_slot: Vec<OutputSlot>,
}

#[derive(Store, Clone, Debug)]
pub struct NodeStore {
    #[store(key: usize = |row| row.id)]
    pub rows: Vec<Node>,
}

#[component]
pub fn NodeContainer(
    #[prop(into)]
    node: AtKeyed<Store<NodeStore>, NodeStore, usize, Vec<Node>>,
    on_start_connection: impl Fn(usize, MouseEvent) + 'static,
    on_end_connection: impl Fn(usize, MouseEvent) + 'static,
) -> impl IntoView {
    let node_id = node.read().id;

    view! {
        <div class="container"
            style:width=move || format!("{}px", node.read().width)
            style:left=move || format!("{}px", node.read().x)
            style:top=move || format!("{}px", node.read().y)
        >
            <div class="title">
                {node.read().label.clone()}
            </div>
            <div class="input_output_pair">
                <div class="input_list">

                    <div class="input" title="Input" />
                    <div class="input" title="Input" />
                    <div class="input"
                        on:mouseup=move |e| on_end_connection(node_id, e)
                        title="Input"
                    />
                </div>

                <div class="output_list">
                    <div class="output" title="Output - drag to connect" />
                    <div class="output"
                        on:mousedown=move |e| on_start_connection(node_id, e)
                        title="Output - drag to connect"
                    />
                </div>
            </div>
        </div>
    }
}
