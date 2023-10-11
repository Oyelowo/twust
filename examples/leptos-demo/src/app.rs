use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use twust::tw;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    // Try to break mistype any of the class name and see what happens.
    // Daisyui classes are also ssupported via a feature flag
    view! {
        <div class=tw!("my-5 mx-auto max-w-3xl text-center")>
            <h2 class=tw!("p-6 text-4xl")>"Tw-macro"</h2>
            <i class=tw!("p-2 text-2xl")>"Check your tailwind classes instantly with tw-macro."</i>
            <p class=tw!("p-2 text-2xl")>"We also support daisyui plugin."</p>

            <div class=tw!("mockup-code w-[50px] [margin:auto]")>
                <pre data-prefix="$"><code>cargo add tw-macro</code></pre>
                <pre data-prefix=">" class=tw!("text-warning")><code>installing...</code></pre>
                <pre data-prefix=">" class=tw!("text-success")><code>Done!</code></pre>
            </div>

            <div class=tw!("stats shadow")>
                <div class=tw!("stat")>
                    <div class=tw!("stat-title")>Total Downloads</div>
                    <div class=tw!("stat-value")>193,245,999</div>
                    <div class=tw!("stat-desc")>74% more than last year</div>
                </div>
             </div>

            <div class=tw!("chat chat-start")>
                <div class=tw!("chat-bubble chat-bubble-primary")>
                    "🤖 What's your name?"
                </div>
            </div>
            <div class=tw!("chat chat-end")>
                <div class=tw!("chat-bubble chat-bubble-secondary")>
                    "🙂 My name is Oyelowo. How can I assist you today?"
                </div>
            </div>
            <div class=tw!("chat chat-start")>
                <div class=tw!("chat-bubble chat-bubble-accent")>
                    "🍯 Favorite type of syrup?"
                </div>
            </div>
            <div class=tw!("chat chat-end")>
                <div class=tw!("chat-bubble chat-bubble-info")>
                    "🍁 Oh, it's definitely Maple syrup!"
                </div>
            </div>
            <div class=tw!("chat chat-start")>
                <div class=tw!("chat-bubble chat-bubble-success")>
                    "⚽ Do you have a favorite sports team?"
                </div>
            </div>
            <div class=tw!("chat chat-end")>
                <div class=tw!("chat-bubble chat-bubble-warning")>
                    "🏒 Absolutely! I'm a big fan of The Oilers!"
                </div>
            </div>
            <div class=tw!("chat chat-start")>
                <div class=tw!("chat-bubble chat-bubble-error")>
                    "🎉 Coool! I'll remember that!"
                </div>
            </div>
        </div>
    }
}
