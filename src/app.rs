use leptos_meta::*;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::types::*;

use crate::components::drag_list::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    title: String,
    complete: bool,
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    let mytodo = Todo{title: title, complete: false,};
    Ok(mytodo)
}
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
        <Title text="Welcome to Leptos"/>

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

    let mut items =  Vec::new();
    
    items.push(ListItem { id: 1, name: "Item 1".to_string() });
    items.push(ListItem { id: 2, name: "Item 2".to_string() });
    items.push(ListItem { id: 3, name: "I'm an item!".to_string() });

    view! { cx,
        <DarkModeToggle prefer_dark=true />
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <button on:click=decrement>"-1"</button>
        <DragList items=items/>
    }
}


#[component]
pub fn DarkModeToggle(cx: Scope, 
    /// Whether the component should initially prefer dark mode.
    #[prop(optional)]
    prefer_dark: bool) -> impl IntoView {
    let (is_dark, set_is_dark) = create_signal(cx, prefer_dark);
   
    let toggle_dark = move |_| {
        log!("Updating state: {}",is_dark.get());
        set_is_dark.update(|dark| *dark = !*dark)
    };

    let color_scheme = move || { 
        if is_dark() { "dark" } else{ "light"}
    }.to_string();

    view! {
        cx,
        <Meta name="color-scheme" content=color_scheme />
        <button on:click=toggle_dark>"Toggle Dark Mode"</button>
    }
}

