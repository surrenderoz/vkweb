use leptos::{component, Scope, IntoView, view};



#[component]
pub fn Hero(cx:Scope) -> impl IntoView {
    view! {cx,
        <>
            <div class="flex items-center flex-col py-20 gap-y-6">
                <p class="text-base text-base text-raven">"When Tech solves its own environmental impact"              
                  </p>
                <h3 class="text-white text-xl">
                "Full Stack Development
                "
                </h3>
                <button class="px-8 py-2 border text-raven border-solid border-white rounded hover:border-dotted">"Let's Talk"</button>
            </div>
        </>

    }
}