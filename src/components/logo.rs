use leptos::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div> 
            <img src="assets/gabelogo.png" alt="gabe-logo" class="w-full"> </img>
        </div> 
       }
}