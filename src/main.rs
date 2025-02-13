use yew::prelude::*;
use yew_router::prelude::*;

// Importar módulos de componentes
mod components {
    pub mod home;
    pub mod about;
}
use components::home::Home;
use components::about::About;

// Definir rutas
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

// Manejador de rutas
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
    }
}

// Componente principal con navegación
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav>
                <ul>
                    <li><Link<Route> to={Route::Home}>{ "Inicio" }</Link<Route>></li>
                    <li><Link<Route> to={Route::About}>{ "Acerca de" }</Link<Route>></li>
                </ul>
            </nav>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

// Función principal
fn main() {
    yew::Renderer::<App>::new().render();
}