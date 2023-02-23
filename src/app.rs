use leptos::*;
use leptos_meta::*;
use leptos_router::*;
// use serde::{Deserialize, Serialize};

use crate::types::*;

use crate::components::dark_mode_toggle::*;
use crate::components::drag_list::*;

// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Todo {
//     title: String,
//     complete: bool,
// }

// #[server(AddTodo, "/api")]
// pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
//     let mytodo = Todo {
//         title: title,
//         complete: false,
//     };
//     Ok(mytodo)
// }

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        //<Meta name="color-scheme" content="dark" />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Test 1"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button#
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let decrement = move |_| set_count.update(|count| *count -= 1);

    //let (item_list, _set_item_list) = create_signal(cx, sample_data_listitems());
    let item_list = sample_data_listitems();
    // fn handle_reorder() {
    // }

    view! { cx,
        <DarkModeToggle prefer_dark=true />
        <h1>"Draggable List Test"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <button on:click=decrement>"-1"</button>
        //<DragList items=items on_reorder=handle_reorder/>
        <DragList items={item_list} />
    }
}

fn sample_data_listitems() -> Vec<ListItem> {
    let mut sample_items = Vec::new();

    sample_items.push(ListItem {
        id: 1,
        name: "Item 1".to_string(),
        ..ListItem::default()
    });
    sample_items.push(ListItem {
        id: 2,
        name: "Item 2".to_string(),
        ..ListItem::default()
    });
    sample_items.push(ListItem {
        id: 3,
        name: "I'm an item!".to_string(),
        ..ListItem::default()
    });
    sample_items.push(ListItem {
        id: 4,
        ..ListItem::default()
    });
    sample_items.push(ListItem {
        id: 5,
        ..ListItem::default()
    });

    sample_items
}
