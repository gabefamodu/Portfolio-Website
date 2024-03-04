use leptos::*;
use crate::components::nav_element::NavElement;


#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="space-x-4 w-full flex bg-slate-200 p-8 sticky top-0">
            <NavElement/>
        </nav> 
    
       }
}