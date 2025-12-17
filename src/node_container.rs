use leptos::*;
use leptos::prelude::*;

use reactive_stores::{AtKeyed, Store};

use crate::graph_container::{Connection, DragState};

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
    pub input_slot: Vec<String>,
    pub output_slot: Vec<String>,
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
) -> impl IntoView {
    let (connecting_from, set_connecting_from) = 
        use_context::<(ReadSignal<Option<(usize, usize)>>, WriteSignal<Option<(usize, usize)>>)>()
        .expect("to have found the setter provided");
    let set_connections = use_context::<WriteSignal<Vec<Connection>>>().expect("");
    let set_drag_state = use_context::<WriteSignal<Option<DragState>>>().expect("");
    let end_connection = move |node_id: (usize, usize)| {
        if let Some(from_id) = connecting_from.get() {
            if from_id != node_id {
                set_connections.update(|conns| {
                    conns.push(Connection {
                        from: from_id,
                        to: node_id,
                    });
                });
            }
            set_connecting_from.set(None);
        }
        set_drag_state.set(None);
        // console_log("End connection");
    };

    let node_id = move || node.read().id;
    view! {
        <div class="container"
            style:width=move || format!("{}px", node.read().width)
            style:left=move || format!("{}px", node.read().x)
            style:top=move || format!("{}px", node.read().y)
        >
            <div class="title">
                {move || node.read().label.clone()}
            </div>
            <div class="input_output_pair">
                <div class="input_list">
                    {move || node.read().input_slot.clone()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, title)| view! { 
                            <div class="input" 
                                on:mouseup=move |e| {
                                    e.stop_propagation();
                                    end_connection((node_id(), idx));
                                }
                                title=title
                            />
                        })
                        .collect::<Vec<_>>()
                    }
                </div>

                <div class="output_list">
                    {move || node.read().output_slot.clone()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, title)| view! { 
                            <div class="output"
                                on:mousedown=move |e| {
                                    e.stop_propagation();
                                    set_connecting_from.set(Some((node_id(), idx)));
                                }
                                title=title
                            />
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            </div>
        </div>
    }
}
