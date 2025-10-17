use std::collections::VecDeque;

use leptos::prelude::*;
use leptos_styling::style_sheet;

style_sheet!(chat_styles, "src/components/chat/chat.module.scss", "chat");

#[derive(Clone)]
pub struct ChatMessage {
    pub text: String,
    pub time: String,
    pub sender: ChatSender,
}

#[derive(Clone, PartialEq)]
pub enum ChatSender {
    Sent,
    Received(String),
}

#[component]
pub fn Chat(
    messages: RwSignal<VecDeque<ChatMessage>>,
    #[prop(optional)] on_fetch_more: Option<Callback<()>>, // Called when scroll reaches top
) -> impl IntoView {
    let chat_ref = NodeRef::<leptos::html::Div>::new();

    // Scroll event handler to fetch more messages when at top
    let handle_scroll = move |_| {
        if let Some(div) = chat_ref.get() {
            let scroll_top = div.scroll_top();
            let max_scroll = div.scroll_height() - div.client_height();
            // If near the top (visual top), scroll_top is close to max_scroll
            log::info!("Scroll top: {}, Max scroll: {}", scroll_top, max_scroll);
            if (max_scroll + scroll_top).abs() < 5 {
                // 5px tolerance
                log::info!("Fetching more messages");
                if let Some(fetch_cb) = &on_fetch_more {
                    fetch_cb.run(());
                }
            }
        }
    };

    view! {
        <div
            class=chat_styles::CHAT
            node_ref=chat_ref
            style="overflow-y: auto; max-height: 600px;"
            on:scroll=handle_scroll
        >
            <For
                each=move || {
                    let mut msgs = messages.get();
                    msgs.into_iter().rev()
                }
                key=|msg| (msg.time.clone(), msg.text.clone()) // You can use a unique id if available
                children=move |msg| {
                    let class = match &msg.sender {
                        ChatSender::Sent => format!("{} {}", chat_styles::MSG, chat_styles::SENT),
                        ChatSender::Received(_) => format!("{} {}", chat_styles::MSG, chat_styles::RCVD),
                    };
                    let time = match &msg.sender {
                        ChatSender::Sent => msg.time.clone(),
                        ChatSender::Received(name) => format!("{} {}", name, msg.time),
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
    }
}
