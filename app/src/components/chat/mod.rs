use std::collections::{HashMap, VecDeque};

use api::server_fn::chat::{
    ChatChannelMessages, ChatMessage, ChatSender, fetch_messages, publish_message,
};
use chrono::{DateTime, Duration, Local, Utc};
use leptos::{prelude::*, task::spawn_local};
use leptos_styling::style_sheet;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        card::{Card, CardBody, CardHeader},
        input_bar::{InputBar, Person},
    },
    contexts::account_context::AccountContext,
};

style_sheet!(chat_styles, "src/components/chat/chat.module.scss", "chat");
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReadersAndWriters {
    readers: HashMap<String, Person>,
    writers: HashMap<String, Person>,
}

#[component]
pub fn Chat(group_id: String) -> impl IntoView {
    let chat_ref = NodeRef::<leptos::html::Div>::new();
    let page_size = 40;
    let offset = RwSignal::new(0);
    let messages = RwSignal::new(VecDeque::new());

    // Resource for the initial fetch
    let initial_messages = Resource::new(
        {
            let group_id = group_id.clone();
            move || group_id.clone()
        },
        move |group_id| {
            let group_id = group_id.clone();
            async move { fetch_messages(group_id, 0, page_size).await.ok() }
        },
    );
    Effect::new(move |_| {
        if let Some(Some(fetched)) = initial_messages.get() {
            let count = fetched.len();
            messages.set(fetched.into_iter().rev().collect());
            offset.set(count as i64);
        }
    });

    let on_fetch_more = {
        let group_id = group_id.clone();
        let messages = messages.clone();
        let offset = offset.clone();
        Callback::new(move |_| {
            let group_id = group_id.clone();
            let messages = messages.clone();
            let offset = offset.clone();
            spawn_local(async move {
                let current_offset = offset.get_untracked();
                if let Ok(fetched) =
                    fetch_messages(group_id.clone(), current_offset, page_size).await
                {
                    let count = fetched.len();
                    if count > 0 {
                        messages.update(|msgs| {
                            for msg in fetched.into_iter().rev() {
                                msgs.push_front(msg);
                            }
                        });
                        offset.set(current_offset + count as i64);
                    }
                }
            });
        })
    };

    // Scroll event handler to fetch more messages when at top
    let handle_scroll = move |_| {
        if let Some(div) = chat_ref.get() {
            let scroll_top = div.scroll_top();
            let max_scroll = div.scroll_height() - div.client_height();
            // If near the top (visual top), scroll_top is close to max_scroll
            if (max_scroll + scroll_top).abs() < 5 {
                // 5px tolerance
                log::info!("Fetching more messages");
                on_fetch_more.run(());
            }
        }
    };
    let account = use_context::<AccountContext>().expect("AccountContext not found");
    let username = move || {
        account
            .user_untracked()
            .and_then(|v| v.username().map(|v| v.to_string()))
    };
    let new_messages =
        leptos_ws::ChannelSignal::<ChatChannelMessages>::new(&group_id).and_then(|signal| {
            signal.on_client(move |msg| match msg {
                ChatChannelMessages::NewMessage(msg) => {
                    messages.update(|msgs| {
                        msgs.push_back(ChatMessage {
                            text: msg.text.clone(),
                            time: msg.time.clone(),
                            sender: if let Some(username) = username()
                                && username == msg.username
                            {
                                ChatSender::Sent
                            } else {
                                ChatSender::Received(msg.username.clone())
                            },
                        });
                    });
                    offset.update(|o| *o += 1)
                }
            })
        });
    let writing = RwSignal::new(false);
    Effect::new(move || {
        writing.track();
        log::info!("Writing effect triggered");
    });
    let writers_readers = leptos_ws::BiDirectionalSignal::<ReadersAndWriters>::new(
        &format!("{}-activity", group_id),
        ReadersAndWriters::default(),
    )
    .unwrap();
    let writers_readers2 = writers_readers.clone();
    let writers_readers3 = writers_readers.clone();

    let writers = Memo::new(move |_| {
        let writers = writers_readers2.read();
        let current_time = Utc::now();
        writers
            .writers
            .values()
            .filter(|v| v.last_activity > current_time - Duration::seconds(20))
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    });
    let readers = Memo::new(move |_| {
        let readers = writers_readers3.read();
        let current_time = Utc::now();
        readers
            .readers
            .values()
            .filter(|v| v.last_activity > current_time - Duration::seconds(20))
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    });
    Effect::new(move |_| {
        let writers_readers = writers_readers.clone();
        set_interval(
            move || {
                writers_readers.update(move |data| {
                    let Some(username) = username() else { return };

                    if writing.get_untracked() {
                        data.readers.remove(&username);
                        data.writers
                            .entry(username.clone())
                            .or_insert(Person {
                                name: username.clone(),
                                last_activity: Utc::now(),
                            })
                            .last_activity = Utc::now();
                    } else {
                        data.writers.remove(&username);
                        data.readers
                            .entry(username.clone())
                            .or_insert(Person {
                                name: username.clone(),
                                last_activity: Utc::now(),
                            })
                            .last_activity = Utc::now();
                    }
                });
            },
            std::time::Duration::from_secs(3),
        );
    });
    view! {
        <div class=chat_styles::CHAT_CONTAINER>
            <div
                class=chat_styles::CHAT
                node_ref=chat_ref
                on:scroll=handle_scroll
            >
                <For
                    each=move || {
                        let mut msgs = messages.get();
                        msgs.into_iter().rev()
                    }
                    key=|msg| msg.clone()
                    children=move |msg| {
                        let class = match &msg.sender {
                            ChatSender::Sent => format!("{} {}", chat_styles::MSG, chat_styles::SENT),
                            ChatSender::Received(_) => format!("{} {}", chat_styles::MSG, chat_styles::RCVD),
                        };
                        let converted: DateTime<Local> = DateTime::from(msg.time);
                        let time = match &msg.sender {
                            ChatSender::Sent => converted.format("%H:%M").to_string(),
                            ChatSender::Received(name) => format!("{} {}", name, converted.format("%H:%M").to_string()),
                        };
                        view! {
                            <div
                                data-time=time
                                class=class
                            >
                                {msg.text.clone()}
                            </div>
                        }
                    }
                />
            </div>
            <InputBar writers readers writing=writing.write_only() on_submit=Callback::new(move |message| spawn_local( {
                let group_id = group_id.clone();
                async move {
                publish_message(group_id, message).await;
            }}))/>
        </div>

    }
}

#[component]
pub fn SelectGroup() -> impl IntoView {
    view! {
        <div class=chat_styles::CHAT_CONTAINER>
            <Card>
                <CardHeader>
                    <h1>"Group not selected"</h1>
                </CardHeader>
                <CardBody>
                    "Please select a group from the left sidebar."
                </CardBody>
            </Card>
        </div>
    }
}
