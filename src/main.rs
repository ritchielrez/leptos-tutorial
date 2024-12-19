use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| { *set_count.write() += 1; }
        class=("custom-btn", move || count.get() % 2 == 1)
        style:width=move || format!("{}px", count.get() + 100)>
            "Click me"
        </button>
        <br/>
        <label>"Count: " {count} <progress max="300" value=count /> </label> // reactive
        // <p>"Count: " {count.get()} </p> // non reactive
        // <p>"Count: " {move || count.get} </p> // reactive, because we are passing a function(closure)
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
