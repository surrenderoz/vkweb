use leptos::{component, Scope, IntoView, view};

use crate::components::home::hero::Hero;

use crate::components::home::about::About;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx, 
      <div class="container mx-auto">
        <Hero />
        <About />
        </div>
    }
}
