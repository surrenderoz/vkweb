use leptos::{component, Scope, IntoView, view, mount_to_body};
mod components;
mod pages;
use components::navbar::Navbar;
use pages::index::*;

#[component]
fn App(cx:Scope) -> impl IntoView {
    view! {
        cx, 
        <div>
            <video autoplay muted loop  class="fixed -z-10">
                <source src="./assets/video.mp4" type="video/mp4" /> 
            </video>
        <Navbar />
        <>
            <Home />
        </>
        </div>
    }
}
fn main() {
    mount_to_body(|cx| view!{cx, 
        <App />
    })
}
