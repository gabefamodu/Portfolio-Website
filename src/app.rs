use leptos::*;

use crate::components::nav_bar::NavBar;
 
#[component]
pub fn App() -> impl IntoView {

// script area where rust goes. 

    view! {
        <NavBar/>
        <main> 
           <h1 class="text-3xl font-bold underline"> hey there </h1> 
        </main>}
 
}
