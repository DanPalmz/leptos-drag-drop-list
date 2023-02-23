use std::fmt::Debug;

// use web_sys::{EventTarget, DataTransfer};
// use leptos::{*, html::Meta, HtmlElement, ev::DragEvent};
use super::super::types::*;
use leptos::{ev::DragEvent, *};

#[component]
pub fn DragList<T>(cx: Scope, items: Vec<T>) -> impl IntoView
where
    T: Listable + Clone + Debug + 'static,
{
    let (itemslist, set_itemslist) = create_signal(cx, items.clone());
    let (droptarget, set_droptarget) = create_signal(cx, None);
    let (dragitem, set_dragitem) = create_signal(cx, None);

    // fn drop_hover(d: Option<i32>, reset: bool) {
    //     match d {
    //         Some(x) => {
    //             log!("Hover over: {}, reset: {}", x, reset);
    //         }
    //         None => {
    //             log!("Hover over: nothing, reset: {}", reset);
    //         }
    //     }
    // }
    // fn drop_hover(item_id: i32, drag: DragState) {
    //     match drag {
    //         DragState::Dragging => set_dragitem.update(|d| *d = Some(item_id)),
    //         DragState::DraggedOver => set_droptarget.update(|d| *d = Some(item_id)),
    //         //log!("Currently over {}", item_id),
    //         _ => {}
    //     }
    //     log!("Currently {} over {}", dragitem.get(), droptarget.get());
    // }

    let drop_hover = |item_id: i32, drag: DragState| {
        match drag {
            DragState::Dragging => set_dragitem(Some(item_id)),
            DragState::DraggedOver => set_droptarget.update(|d| *d = Some(item_id)),
            //log!("Currently over {}", item_id),
            _ => {}
        };
        log!(
            "Currently {} over {}",
            dragitem.get().unwrap_or(99),
            droptarget.get().unwrap_or(99)
        );
    };

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

    // fn display_items(v: Vec<T>) {
    //         v.into_iter().map(|i|
    //             log!("Hello world")
    //          ).collect::<Vec<_>>();
    // }
    // let drag_item_view = items
    //             .into_iter()
    //             .map(|item|
    //                 view! {
    //                     cx,
    //                     <DragItem item=item callback=&(drop_hover as fn(std::option::Option<i32>, bool))/>
    //                 }
    //             )
    //             .collect::<Vec<_>>();
    //   <DragItem item={item} callback={&(drop_hover as fn(std::option::Option<i32>, bool))} />

    view! {
        cx,
        <ul class="moveable" droppable="true">
            <For each=itemslist
                key=|i| i.get_id()
                view=move |cx, item| {
                    view!{cx,
                       // <p>{item.get_name()}</p>
                        <DragItem item={item} callback={drop_hover} />
                    }
                }
            />
            // {drag_item_view}
        </ul>
        <button on:click=reverse_items>"Randomize"</button>
        //<button on:click=add_item>"Add Item"</button>
    }
}

// fn DragItem<T>(cx: Scope, item: T, callback: fn(i32, DragState)) -> impl IntoView
// where
//     T: Listable,

#[component]
fn DragItem<T, CB>(cx: Scope, item: T, callback: CB) -> impl IntoView
where
    T: Listable,
    CB: Fn(i32, DragState) + 'static,
{
    let (dragging, set_dragging) = create_signal(cx, DragState::Normal);
    let (dragged_over, set_dragged_over) = create_signal(cx, DragState::Normal);

    //let dragging_bool = move || dragging.
    let item_id = item.get_id();

    let start_dragging = move |event: DragEvent| {
        //event.prevent_default();

        set_dragging.update(|drag| *drag = DragState::Dragging);
        log!("Dragging item started {:?}", dragging.get());
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
    };

    let ondragenter = move |event: DragEvent| {
        //event.prevent_default();
        callback(item_id, DragState::DraggedOver);

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
