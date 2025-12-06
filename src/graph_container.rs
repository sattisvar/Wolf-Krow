use std::collections::HashMap;

use leptos::*;
use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator};
use wasm_bindgen::prelude::*;

use crate::node_container::{InputSlot, Node, NodeContainer, NodeStore, NodeStoreStoreFields, OutputSlot};

const PORT_VERTICAL_OFFSET: f64 = 50.0;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



#[derive(Clone, Debug)]
struct Connection {
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
struct DragState {
    node_id: usize,
    offset_x: f64,
    offset_y: f64,
}


#[component]
pub fn GraphContainer() -> impl IntoView {
    let nodes_store = Store::new(NodeStore {
        rows: vec![
            Node {
                id: 0,
                x: 100.0,
                y: 100.0,
                width: 150.0,
                label: "Input".to_string(),
                input_slot: vec![InputSlot { id: 0, title: "in1".to_string() }, ],
                output_slot: vec![OutputSlot { id: 0, title: "out1".to_string() }, ],
            },
            Node {
                id: 1,
                x: 400.0,
                y: 150.0,
                width: 150.0,
                label: "Process".to_string(),
                input_slot: vec![InputSlot { id: 0, title: "in1".to_string() }, ],
                output_slot: vec![OutputSlot { id: 0, title: "out1".to_string()}, ],
            },
            Node {
                id: 2,
                x: 700.0,
                y: 100.0,
                width: 150.0,
                label: "Output".to_string(),
                input_slot: vec![InputSlot { id: 0, title: "in1".to_string() }, ],
                output_slot: vec![OutputSlot { id: 0, title: "out1".to_string() }, ],
            },
        ],
    });

    let (connections, set_connections) = signal(Vec::<Connection>::new());
    let (connecting_from, set_connecting_from) = signal(None::<usize>);
    let (temp_line_end, set_temp_line_end) = signal(None::<(f64, f64)>);

    let (drag_state, set_drag_state) = signal(None::<DragState>);

    let next_id = RwSignal::new(3usize);

    let (scale, set_scale) = signal(1.0f64);
    
    let on_mouse_move = move |e: web_sys::MouseEvent| {
        if let Some(state) = drag_state.get() {
            //set_nodes.update(|nodes| {
            if let Some(node) = nodes_store
                .rows()
                .iter_unkeyed()
                .find(|n| n.read().id == state.node_id)
            {
                let mut nw = node.write();
                nw.x = e.client_x() as f64 - state.offset_x;
                nw.y = e.client_y() as f64 - state.offset_y;
            }
        }

        if connecting_from.get().is_some() {
            set_temp_line_end.set(Some((e.client_x() as f64, e.client_y() as f64)));
        }
    };

    

    let on_focus_out = move |_e: web_sys::FocusEvent| {
        set_drag_state.set(None);
        set_temp_line_end.set(None);
        // console_log("Focus out");
    };

    let on_wheel_y = move |e: web_sys::WheelEvent| {
        e.stop_propagation();
        /* console_log(format!("Wheel {} delta x {} y {} z {}",
            scale.get(),
            e.delta_x(),
            e.delta_y(),
            e.delta_z(),
        ).as_str()); */
        set_scale.set(
            (1.0_f64)
                .max(scale.get() + e.delta_y() * -0.005)
                .min(2.0f64),
        );
    };

    let on_mouse_down = move |node_id: usize, e: web_sys::MouseEvent| {
        e.prevent_default();
        use reactive_stores::StoreFieldIterator;

        let mut nodes_val = nodes_store.rows().iter_unkeyed();
        if let Some(node) = nodes_val.find(|n| n.read().id == node_id) {
            let this_node = node.read();
            // console_log(format!("{node_id} is being dragged").as_str());
            set_drag_state.set(Some(DragState {
                node_id,
                offset_x: e.client_x() as f64 - this_node.x,
                offset_y: e.client_y() as f64 - this_node.y,
            }));
        }
    };

    let on_mouse_up = move |_e: web_sys::MouseEvent| {
        set_drag_state.set(None);
        set_temp_line_end.set(None);
        set_connecting_from.set(None);

        // console_log("mouse up");
    };

    let start_connection = move |node_id: usize, e: web_sys::MouseEvent| {
        e.stop_propagation();
        set_connecting_from.set(Some(node_id));
        // console_log("Start connection");
    };

    let end_connection = move |node_id: usize, e: web_sys::MouseEvent| {
        e.stop_propagation();
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

    let add_node = move |_| {
        let id = next_id.get();
        next_id.set(id + 1);
        nodes_store.rows().write().push(Node {
            id,
            x: 200.0 + (id as f64 * 50.0),
            y: 200.0 + (id as f64 * 30.0),
            width: 150.0,
            label: format!("Node {}", id),
            input_slot: vec![InputSlot { id: 0, title: "in1".to_string() }, ],
            output_slot: vec![OutputSlot { id: 0, title: "out1".to_string() }, ],
        });
    };

    view! {
        <div
            style="width: 100vw; height: 100vh; position: relative; overflow: hidden; background: #1a1a2e; cursor: default;"
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:wheel=on_wheel_y
            on:focusout=on_focus_out
            style:transform=move || format!("scale({})", scale.get())
            style:marginTop=move || format!("{}vh", (scale.get() - 1.0) * 50.0)
            style:marginLeft=move || format!("{}vw", (scale.get() - 1.0) * 50.0)
        >
            <button
                on:click=add_node
                style="position: absolute; top: 10px; left: 10px; z-index: 1000; padding: 10px 20px; background: #4a5568; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                "+"
            </button>

            <svg style="position: absolute; width: 100%; height: 100%; pointer-events: none;">
                <defs>
                    <marker id="arrowhead" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
                        <polygon points="0 0, 10 3, 0 6" fill="#64ffda" />
                    </marker>
                </defs>

                {move || {
                    let binding = nodes_store.rows();
                    let nodes_map: HashMap<usize, Node> = binding.iter_unkeyed().map(|n| (n.read().id, n.get())).collect();
                    connections.get().iter().filter_map(|conn| {
                        let from = nodes_map.get(&conn.from)?;
                        let to = nodes_map.get(&conn.to)?;
                        let x1 = from.x + from.width + 15.0;
                        let y1 = from.y + PORT_VERTICAL_OFFSET + 16.0 * 3.0;
                        let x2 = to.x + 5.0;
                        let y2 = to.y + PORT_VERTICAL_OFFSET;
                        let dx = (x2 - x1).abs() * 0.5;
                        let path = format!("M {} {} C {} {}, {} {}, {} {}",
                            x1, y1, x1 + dx, y1, x2 - dx, y2, x2, y2);
                        Some(view! {
                            <path
                                d=path
                                stroke="#64ffda"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#arrowhead)"
                            />
                        })
                    }).collect::<Vec<_>>()
                }}

                {move || {
                    if let (Some(from_id), Some((end_x, end_y))) = (connecting_from.get(), temp_line_end.get()) {
                        let nodes_val = nodes_store.rows();
                        nodes_val.iter_unkeyed().find(|n| n.read().id == from_id).map(|from| {
                            let from = from.get();
                            let x1 = from.x + from.width - 5.0;
                            let y1 = from.y + PORT_VERTICAL_OFFSET;
                            let dx = (end_x - x1).abs() * 0.5;
                            let path = format!("M {} {} C {} {}, {} {}, {} {}",
                                x1, y1, x1 + dx, y1, end_x - dx, end_y, end_x, end_y);
                            view! {
                                <path
                                    d=path
                                    stroke="#64ffda"
                                    stroke-width="2"
                                    fill="none"
                                    stroke-dasharray="5,5"
                                    opacity="0.5"
                                />
                            }
                        })
                    } else {
                        None
                    }
                }}
            </svg>
            <For
                each=move || nodes_store.rows()
                key=|node| node.read().id.clone()
                children=move |node| {
                    let node_id = node.read().id;

                    // console_log(format!("doing node {:?}", node).as_str());
                    view!{
                        <NodeContainer 
                            node=node
                            on_start_connection=start_connection
                            on_end_connection=end_connection
                            on:mousedown=move |e| on_mouse_down(node_id, e)
                            on:mouseup=on_mouse_up
                        >
                            </NodeContainer>
                    }
                }
            />
        </div>
    }
}