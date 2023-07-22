use leptos::{component,Scope, IntoView, view};

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="flex flex-row text-white text-base">
            <div class="basis-1/2 text-center my-auto">
                "Let's get it done!"
            </div>
            <div class="basis-1/2">
                "
                    we transform your idea to real business model and build a realiable product
                    for users that will help users.
                    our expert team will guide you to build your dream project and we will introduce your 
                        ideal product to the world.
                "
            </div>
        </div>
    }
}
