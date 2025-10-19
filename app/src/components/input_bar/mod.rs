use std::time::Duration;

use crate::{
    components::{button::Button, input::InputField},
    contexts::account_context::AccountContext,
};
use chrono::{DateTime, Utc};
use leptos::{ev::KeyboardEvent, prelude::*};
use leptos_icons::Icon;
use leptos_styling::style_sheet;
use serde::{Deserialize, Serialize};
style_sheet!(
    input_bar_styles,
    "src/components/input_bar/input_bar.module.scss",
    "input_bar"
);

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct Person {
    pub name: String,
    pub last_activity: DateTime<Utc>,
}

#[component]
pub fn InputBar(
    #[prop(into)] readers: Signal<Vec<Person>>,
    #[prop(into)] writers: Signal<Vec<Person>>,
    #[prop(into, optional)] on_submit: Option<Callback<String>>,
    #[prop(into)] writing: WriteSignal<bool>,
) -> impl IntoView {
    let message = RwSignal::new(String::new());
    // Timer handle for writing detection
    let writing_timeout: RwSignal<Option<TimeoutHandle>> = RwSignal::new(None);
    let on_user_input = {
        move || {
            writing.set(true);
            // Clear previous timeout
            if let Some(handle) = writing_timeout.write().take() {
                handle.clear();
            }
            // Set a new timeout to set is_writing to false after 2 seconds
            let handle = set_timeout_with_handle(
                move || {
                    writing.set(false);
                },
                Duration::from_secs(2),
            )
            .unwrap();
            writing_timeout.set(Some(handle));
        }
    };
    let account = use_context::<AccountContext>().expect("AccountContext not found");
    Effect::new(move || {
        writing.set(false);
    });
    view! {
        <div class=input_bar_styles::INPUT_BAR>
            <StatusBar readers writers/>


            <div class=input_bar_styles::INPUT_ROW>
            <Suspense>
                {move || account
                    .user()
                    .and_then(|v| v.username().map(|v| v.to_string())).map(|username| view!{
                    <div class=input_bar_styles::AVATAR style=move || format!("background-image: url('https://robohash.org/{username}');")></div>
                })}
            </Suspense>
            <div class=input_bar_styles::INPUT_FIELD>
                <InputField value=message name="message"
                    on:input=move |_| on_user_input()
                    placeholder="Message..." no_bottom_margin=true  {..} on:keydown=move |ev: KeyboardEvent| {
                        on_user_input();

                        if ev.key() == "Enter" && let Some(on_submit) = on_submit{
                            on_submit.run(message.get());
                            message.set(String::new());

                        }
                    } />
            </div>
            <Button variant=crate::components::button::ButtonVariant::Primary center=true {..} on:click=move |_| {
                if let Some(on_submit) = on_submit {
                    on_submit.run(message.get());
                    message.set(String::new());
                }
            }>
                <Icon icon=icondata::IoSend/>
            </Button>
            </div>
        </div>

    }
}

#[component]
pub fn StatusBar(
    #[prop(into)] readers: Signal<Vec<Person>>,
    #[prop(into)] writers: Signal<Vec<Person>>,
) -> impl IntoView {
    let dot_frames = [".", "..", "..."];
    let dot_index = RwSignal::new(0);
    Effect::new(move |_| {
        set_interval(
            move || dot_index.update(|i| *i = (*i + 1) % 3),
            std::time::Duration::from_millis(400),
        )
    });
    let status_text = move || {
        let writers = writers.read();
        let readers = readers.read();

        let (names, verb) = if !writers.is_empty() {
            let names = writers.iter().map(|p| p.name.clone()).collect::<Vec<_>>();
            let animation_index = dot_index.read();
            let dot_frame = dot_frames[*animation_index];
            let verb = if names.len() == 1 {
                format!("is writing{dot_frame}")
            } else {
                format!("are writing{dot_frame}")
            };
            (names, verb)
        } else {
            let names = readers.iter().map(|p| p.name.clone()).collect::<Vec<_>>();
            let verb = if names.len() == 1 {
                "is looking"
            } else {
                "are looking"
            };
            (names, verb.to_string())
        };

        if names.is_empty() {
            "No one is here".to_string()
        } else {
            format!("{} {}", names.join(" and "), verb)
        }
    };

    // Combine avatars, avoiding duplicates by avatar_url
    let all_people = move || {
        let mut seen = std::collections::HashSet::new();
        let mut combined = Vec::new();
        for p in readers.read().iter().chain(writers.read().iter()) {
            if seen.insert(format!("https://robohash.org/{}", p.name)) {
                combined.push(p.clone());
            }
        }
        combined
    };

    view! {
        <div class=input_bar_styles::STATUS>
            <div class=input_bar_styles::AVATARS>
            {move || {
                let people = all_people();
                let count = people.len();
                view! {
                    { people.iter().take(2).map(|person| view! {
                        <div class=format!("{} {}",input_bar_styles::AVATAR, input_bar_styles::SMALL) style=format!("background-image: url('https://robohash.org/{}');",person.name)></div>
                    }).collect_view()}
                    {
                        if count > 2 {
                            Some(view! {
                                <div class=format!("{} {}",input_bar_styles::AVATAR, input_bar_styles::SMALL) style="display: flex; align-items: center; justify-content: center; background: var(--gray-200);">
                                    ..
                                </div>
                            })
                        } else {
                            None
                        }
                    }
                }
            }}
            </div>
            <p>{status_text}</p>
        </div>
    }
}
