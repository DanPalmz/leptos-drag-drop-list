use super::super::types::*;
use super::drag_list::*;
use leptos::*;
// use web_sys::{DataTransfer, DragEvent, Event};
use web_sys::DragEvent;

#[component]
pub fn DragItem<T>(cx: Scope, item: T) -> impl IntoView
where
    T: Listable,
{
    let (is_dragging, set_dragging) = create_signal(cx, false);
    let (is_dragged_over, set_dragged_over) = create_signal(cx, false);

    let dragitem_setter = use_context::<DraggingItemContext>(cx).unwrap().0;
    let droptarget_setter = use_context::<DropTargetContext>(cx).unwrap().0;
    let drag_drop_event = use_context::<DragDropContext>(cx).unwrap().0;

    let item_id = item.get_id();

    let start_dragging = move |ev: DragEvent| {
        let dt = ev.data_transfer().unwrap();
        dt.set_drop_effect("move");
        dt.clear_data()
            .map_err(|err| log!("Error clearing DT data {:?}", err))
            .ok();
        dt.set_data("text", item_id.to_string().as_str())
            .map_err(|err| log!("Error setting DT value {:?}", err))
            .ok();
        set_dragging.update(|drag| *drag = true);
        dragitem_setter.set(Some(item_id));
    };

    let end_dragging = move |event: DragEvent| {
        event.prevent_default();
        dragitem_setter.set(None);
        //log!("Dragging state currently {:?}", dragging.get());
        //log!("drag ended for item{}", item_id);
        set_dragging.update(|d| *d = false);
    };

    let drop = move |event: DragEvent| {
        let dt = event.data_transfer().unwrap();
        dt.set_drop_effect("move");
        let data = dt.get_data("text");
        let source = data.unwrap().parse::<i32>().unwrap();

        let result = Some((source, item_id));
        //log!("dropped item fired {:?}", result);
        drag_drop_event.set(result);

        // Clean up
        set_dragged_over.update(|d| *d = false);
        droptarget_setter.set(None);
    };

    let ondragenter = move |event: DragEvent| {
        event.prevent_default();
        let dt = event.data_transfer().unwrap();
        dt.set_drop_effect("move");

        droptarget_setter.set(Some(item_id));

        set_dragged_over.update(|d| *d = true);
    };

    let ondragleave = move |_| {
        set_dragged_over.update(|d| *d = false);
        droptarget_setter.set(None);
    };

    let ondragover = move |event: DragEvent| {
        event.prevent_default();
        // log!("On drag over");
        let dt = event.data_transfer().unwrap();
        dt.set_drop_effect("move");
    };

    view! {
        cx,
        <li
            class:dragging=is_dragging
            class:over=is_dragged_over
            on:drop=drop
            on:dragstart=start_dragging
            on:dragend=end_dragging
            on:dragenter=ondragenter
            on:dragleave=ondragleave
            on:dragover=ondragover
            id=item_id
            draggable="true">
        { item.get_name() }
        </li>
    }
}
