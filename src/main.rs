mod app; // I want to start writing code in another file. 
mod components; // Use is I want to use code from another file in this file. 
use app::App;
use leptos::*;
 

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

