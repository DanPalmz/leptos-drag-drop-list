use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::types::*;

use crate::components::dark_mode_toggle::*;
use crate::components::drag_list::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Leptos Dragble List Test"/>

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
    // Initialize list with sample data
    let item_list = sample_data_listitems();

    view! { cx,
        <DarkModeToggle prefer_dark=true />
        <h1>"Leptos Dragable List Test"</h1>
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
