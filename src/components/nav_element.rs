use leptos::*;

#[component]
pub fn NavElement() -> impl IntoView { 
    let css = "hover:text-white text text-lg"; 
    view! {
            <a class = css href="#">"About Me"</a> 
            <a class = css href="#">"LinkedIn"</a> 
            <a class = css href="#">"Projects"</a>
            <a class = css href="#">"Resume"</a> 
            <a class = css href="#">"Get In Touch"</a> 

       }
} 

