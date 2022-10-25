use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class="alert alert-primary" role="alert">
                {"A simple primary alert from Bootstrap 5.1.3! Check it out!"}
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}