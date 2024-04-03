use leptos::*;
use crate::components::nav_element::NavElement;
// use crate::components::logo::Logo;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="space-x-4 w-full flex bg-slate-200 p-4 sticky top-0 justify-center"> 
            <NavElement/>
        </nav> 

       }
} // hello