use leptos::{component, Scope, IntoView, view};

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! {cx, 
       <div class="sticky top-0 left-0">
        <div class="flex justify-between py-[30px] container mx-auto text-raven cursor-pointer">
            <h3>"LOGO"</h3>
            <h3 class="flex gap-x-[10px]">
                <p>"About"</p>
                <p>"Blog"</p>
            </h3>
            <h3>"SUBMENU"</h3>
        </div>
    </div>
    }
}