
use leptos::prelude::*;
use stylance::import_crate_style;
import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(style, "./src/components/home_page/style.module.scss");

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    view! {
        <h1 class=main_style::centered>"Welcome to Leptos!"</h1>
        <div class=main_style::centered> 
            <button 
                on:click=on_click
                class=style::red_centered_button
                >"Click Me: " {count}</button>
        </div>
        
    }
}