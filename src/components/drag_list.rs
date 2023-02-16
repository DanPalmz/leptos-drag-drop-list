// use web_sys::{EventTarget, DataTransfer};
// use leptos::{*, html::Meta, HtmlElement, ev::DragEvent};
use leptos::{*,ev::DragEvent};
use super::super::types::*;

#[component]
pub fn DragList<T>(cx: Scope, items: Vec<T>) -> impl IntoView where T: Listable {

    // let mut items =  Vec::new();
    
    // items.push(ListItem { id: (1), name: ("Item 1".to_string()) });
    // items.push(ListItem { id: 2, name: "Item 2".to_string() });

    let display_items = items
                .into_iter()
                .map(|item|
                    view! {
                        cx,
                        <DragItem item=item />
                    }
                )
                .collect::<Vec<_>>();

    view!{
        cx,
        <ul class="moveable" droppable="true">
            {display_items}
        </ul>
    }
}

#[component]
fn DragItem<T>(cx: Scope, item: T) -> impl IntoView where T: Listable {
    let (dragging, set_dragging) = create_signal(cx, false);
    let (dragged_over, set_dragged_over) = create_signal(cx, false);

    let start_dragging= move |event: DragEvent| {
        log!("dragging item started");
        set_dragging.update(|drag| *drag = true);

        if let Some(target) = event.current_target() {
            log!("have target");
        }
    };

    let drop = move |event| {
        log!("dropped item!");
      //  set_dragging.update(|drag| *drag = false);
    };
    
    let end_dragging = move |event| {
        log!("drag ended for item");
        set_dragging.update(|drag| *drag = false);
    };

    let ondragenter = move |event| {
        set_dragged_over.update(|drag| *drag = true);
    };

    let ondragleave = move |event| {
        set_dragged_over.update(|drag| *drag = false);
    };

    view! {
        cx,
        <li
        class:dragging=dragging
        class:over=dragged_over
        on:drop=drop
        on:dragstart=start_dragging
        on:dragend=end_dragging
        on:dragenter=ondragenter
        on:dragleave=ondragleave
            id=item.get_id()
            draggable="true">
        { item.get_name() }
        </li>
    }
}