use leptos::*;

use crate::components::navbar::NavBar;

#[component]
pub fn App() -> impl IntoView {

// script area where rust goes. 

    view! {
            <nav class="flex"> 
              <NavBar/>
            </nav> 
        <main> 
    
           <h1 class="text-3xl font-bold underline"> 
           hey there </h1> 
        </main>}
 
}
