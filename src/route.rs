use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/news/:id")]
    News { id: u8 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    use crate::comp::{Home, News, NotFound};

    match routes {
        Route::Home => html! { <Home /> },
        Route::News { id } => html! { <News id={id} />},
        Route::NotFound => html! { <NotFound /> },
    }
}
