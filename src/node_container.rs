use leptos::*;
use leptos::prelude::*;

use reactive_stores::{AtKeyed, Store};
use web_sys::MouseEvent;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub label: String,
    pub width: f64,
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
        <div
            style="position: absolute; background: #16213ea0; border: 2px solid #0f3460; border-radius: 8px; padding: 10px; cursor: move; user-select: none;"
            style:width=move || format!("{}px", node.read().width)
            style:left=move || format!("{}px", node.read().x)
            style:top=move || format!("{}px", node.read().y)
        >
            <div style="color: #e94560; font-weight: bold; margin-bottom: 8px;">
                {node.read().label.clone()}
            </div>

            <div style="display: flex; justify-content: space-between; margin-top: 10px;">
                <div
                    style="width: 16px; height: 16px; background: #64ffda; border-radius: 50%;"
                    on:mouseup=move |e| on_end_connection(node_id, e)
                    title="Input"
                />
                <div
                    style="width: 16px; height: 16px; background: #64ffda; border-radius: 50%; cursor: pointer;"
                    on:mousedown=move |e| on_start_connection(node_id, e)
                    title="Output - drag to connect"
                />
            </div>
        </div>
    }
}
