use yew::prelude::*;

use yew_hooks::use_counter;

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_counter(0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };
    let onincreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase_by(10))
    };
    let ondecreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease_by(10))
    };
    let onsetmax = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(100))
    };
    let onsetmin = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(-100))
    };
    let onreset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.reset())
    };

    html! {
        <>
        <div>
        <h1>{"Rust Counter App"}</h1>
        <div class="main-container">
            <div>
            <button onclick={onincrease}>{ "+1" }</button>
            </div>
            <div><p>{*counter}</p></div>
            <div>
            <button onclick={ondecrease}>{ "-1" }</button>
            </div>
        </div>
        <div class="main-container">
            <div>
            <button onclick={onsetmax}>{ "set" }</button>
            </div>
            <div>
            <button onclick={onincreaseby}>{"+10"}</button>
            </div>
            <div>
            <button onclick={onreset}>{"Reset"}</button>
            </div>
            <div>
            <button onclick={ondecreaseby}>{"-10"}</button>
            </div>
            <div>
            <button onclick={onsetmin}>{"set"}</button>
            </div>
        </div>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<Counter>::new().render();
}