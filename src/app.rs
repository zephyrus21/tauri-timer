use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    components::{timer_controls::TimerControls, timer_display::TimerDisplay},
    helpers::format_time,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SetTitleArgs<'a> {
    title: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimerState {
    Paused,
    Running,
    Break,
}

fn get_tray_title(timer_state: TimerState, timer_duration: u32, session_length: u32) -> String {
    match timer_state {
        TimerState::Paused => "Paused".to_string(),
        TimerState::Running => {
            if timer_duration > session_length {
                format!("Session finished: {}", format_time(timer_duration))
            } else {
                format!(
                    "Session in progress: {}",
                    format_time(session_length - timer_duration)
                )
            }
        }
        TimerState::Break => {
            if timer_duration > session_length {
                format!("Break finished: {}", format_time(timer_duration))
            } else {
                format!(
                    "Break in progress: {}",
                    format_time(session_length - timer_duration)
                )
            }
        }
    }
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

            let (timer_duration, timer_state, session_length) = props.clone();

            spawn_local(async move {
                let args = to_value(&SetTitleArgs {
                    title: &get_tray_title(*timer_state, *timer_duration, *session_length),
                })
                .unwrap();

                invoke("set_title", args).await;
            });

            move || timeout.cancel();
        },
    );

    html! {
        <main class={classes!("flex", "flex-col", "items-center", "justify-center", "h-screen")}>
            <TimerDisplay
                timer_state={timer_state.clone()}
                timer_duration={timer_duration.clone()}
                session_length={session_length.clone()}
            />
            <TimerControls
                timer_state={timer_state.clone()}
                timer_duration={timer_duration.clone()}
                session_length={session_length.clone()}
            />
        </main>
    }
}

// {
//     let id = some_dep.id(); // Have to extract it in advance, some_dep is moved already in the second argument
//     use_effect_with_dep(move |_| { todo!(); drop(some_dep); }, id);
// }

// use_effect_with(some_dep.id(), move |_| { todo!(); drop(some_dep); });
