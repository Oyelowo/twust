mod app;

use app::*;
use leptos::*;

use leptos_router::components::Router;

pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! {
            <Router>
                <Home />
            </Router>
        }
    })
}
