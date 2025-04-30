use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="w-fit mx-auto mt-5 p-2 rounded-md bg-linear-to-b from-stone-50 to-stone-300">
            <h1 class="text-xl">
                "frust"
            </h1>
        </div>
    }
}