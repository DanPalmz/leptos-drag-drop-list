use std::fmt::Debug;
use std::thread::current;

use super::super::types::*;
use super::drag_item::*;
use leptos::ev::drag;
use leptos::*;

#[derive(Copy, Clone)]
pub struct DraggingItemContext(pub WriteSignal<Option<i32>>);

#[derive(Copy, Clone)]
pub struct DropTargetContext(pub WriteSignal<Option<i32>>);

#[derive(Copy, Clone)]
pub struct DragDropContext(pub WriteSignal<Option<(i32, i32)>>);

#[component]
pub fn DragList<T>(cx: Scope, items: Vec<T>) -> impl IntoView
where
    T: Listable + Clone + Debug + 'static,
{
    let (itemslist, set_itemslist) = create_signal(cx, items);
    let (droptarget, set_droptarget) = create_signal(cx, None);
    let (draggingitem, set_dragitem) = create_signal(cx, None);
    let (drag_drop, set_drag_drop) = create_signal(cx, None);

    provide_context(cx, DraggingItemContext(set_dragitem));
    provide_context(cx, DropTargetContext(set_droptarget));
    provide_context(cx, DragDropContext(set_drag_drop));

    let move_item = move |from_id: i32, to_id: i32| {
        log!("Moving item from {} to {}", from_id, to_id);

        let il = itemslist.get().clone();

        let from_index = il.iter().position(|i| i.get_id() == from_id).unwrap();
        let to_index = il.iter().position(|i| i.get_id() == to_id).unwrap();

        log!("Found at index (from) {} (to) {}", from_index, to_index);

        set_itemslist.update(|mut_il| {
            let removed_item = mut_il.remove(from_index);
            mut_il.insert(to_index, removed_item);
        });
    };

    let handle_drag_drop = create_memo(cx, move |_| {
        let drag_drop_status = drag_drop.get();

        match (drag_drop_status) {
            None => false,
            Some((from, to)) => {
                if from != to {
                    // Prevent updates refiring events while mutating
                    cx.untrack(|| move_item(from, to));
                }
                true
            }
        }
    });

    // UI text
    let dragging_item_memo_from_cx = create_memo(cx, move |_| {
        let current_state = draggingitem();
        //log!("Dragging CX Fired with {:?}", current_state);
        match current_state {
            Some(x) => x.to_string(),
            _ => "nothing".to_string(),
        }
    });

    let droptarget_memo_from_cx = create_memo(cx, move |_| {
        let current_state = droptarget();
        //log!("Droptarget CX Fired with {:?}", current_state);
        match current_state {
            Some(x) => x.to_string(),
            _ => "nothing".to_string(),
        }
    });

    let reverse_items = move |_| {
        set_itemslist.update(|il| {
            let mut v: Vec<T> = vec![];

            let len = il.len();

            for i in 0..len {
                log!("{}", i);
                if let Some(p) = il.pop() {
                    v.push(p);
                }
            }
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
                        <DragItem item={item} />
                    }
                }
            />

        </ul>
        <button on:click=reverse_items>"Reverse List"</button>
        <p>"Currently dragging " {dragging_item_memo_from_cx}</p>
        <p>"Currently over " {droptarget_memo_from_cx}</p>
    }
}
