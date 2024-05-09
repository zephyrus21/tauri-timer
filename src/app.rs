use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::components::timer_display::TimerDisplay;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimerState {
    Paused,
    Running,
    Break,
}

#[function_component(App)]
pub fn app() -> Html {
    let session_length = use_state(|| 25 * 60);
    let timer_duration = use_state(|| 0);
    let timer_state = use_state(|| TimerState::Paused);

    use_effect_with(
        (
            timer_duration.clone(),
            timer_state.clone(),
            session_length.clone(),
        ),
        move |props| {
            let (timer_duration, timer_state, _) = props.clone();

            let timeout = Timeout::new(1_000, move || {
                if *timer_state != TimerState::Paused {
                    timer_duration.set(*timer_duration + 1);
                }
            });

            move || timeout.cancel();

            // let (timer_duration, timer_state, session_length) = props.clone();
        },
    );

    html! {
        <main class={classes!("flex", "flex-col", "items-center", "justify-center", "h-screen")}>
            <TimerDisplay
                timer_state={timer_state.clone()}
                timer_duration={timer_duration.clone()}
                session_length={session_length.clone()}
            />
        </main>
    }
}
