use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Welcome to Blueysh" />
        <Router>
            <Routes>
                <Route path="" view=move |cx| view! { cx, <HomePage /> } />
            </Routes>
        </Router>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <main class="container">
            <h1>"Bluey & Bingo Color Palette"</h1>
            <div class="palette">
            <div class="color" style="background-color: var(--bluey-darker);">"Bluey Darker"</div>
            <div class="color" style="background-color: var(--bluey-dark);">"Bluey Dark"</div>
            <div class="color" style="background-color: var(--bluey-medium);">"Bluey Medium"</div>
            <div class="color" style="background-color: var(--bluey-light);">"Bluey Light"</div>
            <div class="color" style="background-color: var(--bluey-lightest);">"Bluey Lightest"</div>
            <div class="color" style="background-color: var(--bluey-accent);">"Bluey Accent"</div>
          </div>
          <div class="palette">
            <div class="color" style="background-color: var(--bingo-dark);">"Bingo Dark"</div>
            <div class="color" style="background-color: var(--bingo-medium);">"Bingo Medium"</div>
            <div class="color" style="background-color: var(--bingo-light);">"Bingo Light"</div>
          </div>
        </main>
        <span id="forkongithub"><a href="https://github.com/ngmisl/blueyish" target="_blank">"Fork me"</a></span>
    }
}
