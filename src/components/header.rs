use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar bg_black">
            <div class="container-fluid">
                <a class="navbar-brand text-white" href="#">{"User List"}</a>
            </div>
        </nav>
    }
}
