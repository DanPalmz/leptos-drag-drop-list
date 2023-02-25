use super::super::types::*;
use super::drag_list::*;
use leptos::{html::Data, svg::use_, *};
use web_sys::{DataTransfer, DragEvent, Event, HtmlLiElement};

#[component]
pub fn DragItem<T>(cx: Scope, item: T) -> impl IntoView
where
    T: Listable,
    //CB: Fn(i32, DragState),
{
    let (dragging, set_dragging) = create_signal(cx, DragState::Normal);
    let (dragged_over, set_dragged_over) = create_signal(cx, DragState::Normal);

    let dragitem_setter = use_context::<DraggingItemContext>(cx).unwrap().0;
    let droptarget_setter = use_context::<DropTargetContext>(cx).unwrap().0;
    let drag_drop_event = use_context::<DragDropContext>(cx).unwrap().0;

    //let dragging_bool = move || dragging.
    let item_id = item.get_id();

    let start_dragging = move |ev: DragEvent| {
        // let target = &ev.target();
        // let info = target.as_ref().unwrap();
        //let info = event_target_value(ev.target().as_ref().unwrap());
        //let myev = event_::<DataTransfer>(&ev.data_transfer());
        let dt = ev.data_transfer().unwrap();
        dt.set_drop_effect("move");
        dt.clear_data();
        dt.set_data("text", item_id.to_string().as_str());

        //dt.
        //unwrap().value_of();
        //log!("{:?}", info);
        //let target = event_target::<HtmlInputElement>(&ev);
        //ev.prevent_default();
        //ev.init_drag_event(type_)
        set_dragging.update(|drag| *drag = DragState::Dragging);
        //log!("Dragging item started {:?}", dragging.get());
        dragitem_setter.set(Some(item_id));
    };

    let drop = move |event: DragEvent| {
        //event.prevent_default();
        let dt = event.data_transfer().unwrap();
        dt.set_drop_effect("move");
        let data = dt.get_data("text");
        let source = data.unwrap().parse::<i32>().unwrap();

        let result = Some((source, item_id));
        log!("dropped item fired {:?}", result);
        drag_drop_event.set(result);
        //  set_dragging.update(|drag| *drag = false);

        // Clean up
        set_dragged_over.update(|d| *d = DragState::Normal);
        droptarget_setter.set(None);
    };

    let end_dragging = move |event: DragEvent| {
        event.prevent_default();
        dragitem_setter.set(None);
        //log!("Dragging state currently {:?}", dragging.get());
        //log!("drag ended for item{}", item_id);
        set_dragging.update(|d| *d = DragState::Normal);

        // if let Some(_) = event.current_target() {
        //     log!(
        //         "Let go of target {:?}",
        //         event.current_target().unwrap().value_of()
        //     );
        // }
        // log!("Dragging state finally {:?}", dragging.get());
        //dragitem_setter.set(None);
    };

    let ondragenter = move |event: DragEvent| {
        event.prevent_default();
        let dt = event.data_transfer().unwrap();
        dt.set_drop_effect("move");
        //callback(item_id, DragState::DraggedOver);
        droptarget_setter.set(Some(item_id));

        set_dragged_over.update(|d| *d = DragState::DraggedOver);
        // if let Some(_) = event.current_target() {
        //     log!(
        //         "DragEnter {}: {:?}",
        //         item_id,
        //         event.current_target().unwrap()
        // );
        // }
    };

    let ondragleave = move |_| {
        set_dragged_over.update(|d| *d = DragState::Normal);
        droptarget_setter.set(None);
    };

    let dragging_memo = create_memo(cx, move |_| {
        let current_state = dragging();
        //log!("Dragging Memo Fired with {:?}", current_state);
        match current_state {
            DragState::Dragging => true,
            _ => false,
        }
    });
    let ondragover = move |event: DragEvent| {
        event.prevent_default();
        // log!("On drag over");
        let dt = event.data_transfer().unwrap();
        dt.set_drop_effect("move");
    };

    let dragged_over_memo = move || {
        let current_state = dragged_over();
        //log!("Dragged Over Memo Fired with {:?}", current_state);
        match current_state {
            DragState::DraggedOver => true,
            _ => false,
        }
    };

    view! {
        cx,
        <li
            class:dragging=dragging_memo
            class:over=dragged_over_memo
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
