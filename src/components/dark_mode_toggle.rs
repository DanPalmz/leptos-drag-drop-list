use leptos::*;
use leptos_meta::*;

#[component]
pub fn DarkModeToggle(
    cx: Scope,
    /// Whether the component should initially prefer dark mode.
    #[prop(optional)]
    prefer_dark: bool,
) -> impl IntoView {
    let (is_dark, set_is_dark) = create_signal(cx, prefer_dark);

    let toggle_dark = move |_| {
        log!("Updating state: {}", is_dark.get());
        set_is_dark.update(|dark| *dark = !*dark)
    };

    let color_scheme = move || {
        {
            if is_dark() {
                "dark"
            } else {
                "light"
            }
        }
        .to_string()
    };

    view! {
        cx,
        <Meta name="color-scheme" content=color_scheme />
        <button on:click=toggle_dark>"Toggle Dark Mode"</button>
    }
}
