use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

mod components;

mod pages;
use pages::contribution::Contribution;
use pages::home::Home;
use crate::components::jwt_context::JWTProvider;

mod utilities;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/events/:id")]
    Event { id: usize },
}

pub enum Msg {}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <JWTProvider>
            <BrowserRouter>
                <main>
                        <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
            </JWTProvider>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home/> }
        }
        Route::Event { id } => {
            html! {
                 <div class="container mb-2">
                    <Contribution id={ id }/>
                </div>
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
