use leptos::*;
use crate::components::logo::Logo;
 
#[component]
pub fn App() -> impl IntoView {

// script area where rust goes. 

    view! {
        <Logo/>
        <div class="flex items-center justify-center"> 
        <button class="w-200 h-200 bg-blue-500 text-white px-4 py-2 rounded-md hover:back transition duration-300 ease-in-out hover:bg-blue-700"> 
            <p> "View Portfolio" </p> 
        </button> 
        </div> 
    
        <main> 
        </main>}
 
}
