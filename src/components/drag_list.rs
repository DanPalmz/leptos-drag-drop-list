use std::fmt::Debug;

// use web_sys::{EventTarget, DataTransfer};
// use leptos::{*, html::Meta, HtmlElement, ev::DragEvent};
use super::super::types::*;
use leptos::{ev::DragEvent, *};

#[derive(Copy, Clone)]
struct DraggingContext(WriteSignal<Option<i32>>);

#[derive(Copy, Clone)]
struct DropTargetContext(WriteSignal<Option<i32>>);

#[component]
pub fn DragList<T>(cx: Scope, items: Vec<T>) -> impl IntoView
where
    T: Listable + Clone + Debug + 'static,
{
    let (itemslist, set_itemslist) = create_signal(cx, items);
    let (droptarget, set_droptarget) = create_signal(cx, None);
    let (draggingitem, set_dragitem) = create_signal(cx, None);

    provide_context(cx, DraggingContext(set_dragitem));
    provide_context(cx, DropTargetContext(set_droptarget));

    let dragging_memo_from_cx = create_memo(cx, move |_| {
        let current_state = draggingitem();
        log!("Dragging CX Fired with {:?}", current_state);
        match current_state {
            Some(x) => x.to_string(),
            _ => "nothing".to_string(),
        }
    });

    let droptarget_memo_from_cx = create_memo(cx, move |_| {
        let current_state = droptarget();
        log!("Droptarget CX Fired with {:?}", current_state);
        match current_state {
            Some(x) => x.to_string(),
            _ => "nothing".to_string(),
        }
    });

    let reverse_items = move |_| {
        set_itemslist.update(|il| {
            let mut v: Vec<T> = vec![];

            let len = il.len();

            log!("We have {} in list", len);
            for i in 0..len {
                log!("{}", i);
                if let Some(p) = il.pop() {
                    v.push(p);
                }
            }
            log!("New list has {}", v.len());
            log!("{:?}", v);
            *il = v
        })
    };

    view! {
        cx,
        <ul class="moveable" droppable="true">
            <For each=itemslist
                key=|i| i.get_id()
                view=move |cx, item| {
                    view!{cx,
                       // <p>{item.get_name()}</p>
                        //<DragItem item={item} callback={drop_hover} />
                        <DragItem item={item} />
                    }
                }
            />
            // {drag_item_view}
        </ul>
        <button on:click=reverse_items>"Reverse List"</button>
        <p>"Currently dragging " {dragging_memo_from_cx}</p>
        <p>"Currently over " {droptarget_memo_from_cx}</p>
    }
}

// fn DragItem<T>(cx: Scope, item: T, callback: fn(i32, DragState)) -> impl IntoView
// where
//     T: Listable,
// fn DragItem<T, CB>(cx: Scope, item: T, callback: CB) -> impl IntoView

#[component]
fn DragItem<T>(cx: Scope, item: T) -> impl IntoView
where
    T: Listable,
    //CB: Fn(i32, DragState),
{
    let (dragging, set_dragging) = create_signal(cx, DragState::Normal);
    let (dragged_over, set_dragged_over) = create_signal(cx, DragState::Normal);

    let dragitem_setter = use_context::<DraggingContext>(cx).unwrap().0;
    let droptarget_setter = use_context::<DropTargetContext>(cx).unwrap().0;

    //let dragging_bool = move || dragging.
    let item_id = item.get_id();

    let start_dragging = move |event: DragEvent| {
        //event.prevent_default();

        set_dragging.update(|drag| *drag = DragState::Dragging);
        log!("Dragging item started {:?}", dragging.get());
        dragitem_setter.set(Some(item_id));
        // if let Some(_) = event.current_target() {
        //     log!("have target {:?}",event.current_target().unwrap().value_of());
        // }
    };

    let drop = move |event: DragEvent| {
        event.prevent_default();
        log!("dropped item fired {}", item_id);
        //  set_dragging.update(|drag| *drag = false);
    };

    let end_dragging = move |event: DragEvent| {
        event.prevent_default();
        log!("Dragging state currently {:?}", dragging.get());
        log!("drag ended for item{}", item_id);
        set_dragging.update(|d| *d = DragState::Normal);

        if let Some(_) = event.current_target() {
            log!(
                "Let go of target {:?}",
                event.current_target().unwrap().value_of()
            );
        }
        log!("Dragging state finally {:?}", dragging.get());
        dragitem_setter.set(None);
    };

    let ondragenter = move |event: DragEvent| {
        //event.prevent_default();
        //callback(item_id, DragState::DraggedOver);
        droptarget_setter.set(Some(item_id));

        set_dragged_over.update(|d| *d = DragState::DraggedOver);
        if let Some(_) = event.current_target() {
            log!(
                "DragEnter {}: {:?}",
                item_id,
                event.current_target().unwrap()
            );
        }
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
            id=item_id
            draggable="true">
        { item.get_name() }
        </li>
    }
}
