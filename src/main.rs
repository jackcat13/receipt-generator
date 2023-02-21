mod components;

use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::helpers::{facebook_link, github_link, twitter_link, youtube_link};
use components::receipt_generator_component::ReceiptGeneratorComponent;
use components::social_media_block_component::SocialMediaBlockComponent;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn home() -> Html {
    html! {
        <>
            <h1>{"Receipt generator"}</h1>
            <div id="receiptGenerator">
                <ReceiptGeneratorComponent />
            </div>
        </>
    }
}

fn footer() -> Html {
    html! {
        <footer><div>
            <SocialMediaBlockComponent social_media_block = { youtube_link() } />
            <SocialMediaBlockComponent social_media_block = { github_link() } />
            <SocialMediaBlockComponent social_media_block = { twitter_link() } />
            <SocialMediaBlockComponent social_media_block = { facebook_link() } />
        </div></footer>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
            {footer()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
