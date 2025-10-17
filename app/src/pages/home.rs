use std::collections::VecDeque;

use leptos::prelude::*;

use crate::components::{
    chat::{Chat, ChatMessage, ChatSender},
    groups::{Group, Groups},
};

#[component]
pub fn HomePage() -> impl IntoView {
    let example_messages: VecDeque<ChatMessage> = vec![
        ChatMessage {
            text: "Hi!\nWhat's up?".to_string(),
            time: "16:35".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Hi dear!\nDoing some CSS research, you?".to_string(),
            time: "16:36".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Also learning some cool CSS stuff 🦄".to_string(),
            time: "16:38".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "!!".to_string(),
            time: "16:38".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Up for a coffee today? ☕".to_string(),
            time: "16:38".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "It would be a pleasure!".to_string(),
            time: "16:40".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "😍".to_string(),
            time: "16:40".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Are you coming to the meetup tomorrow?".to_string(),
            time: "17:01".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Yes! Looking forward to it.".to_string(),
            time: "17:02".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Great, see you there.".to_string(),
            time: "17:03".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "By the way, did you finish the report?".to_string(),
            time: "17:05".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Almost done, will send it tonight.".to_string(),
            time: "17:06".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Awesome, thanks!".to_string(),
            time: "17:07".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "No problem 😊".to_string(),
            time: "17:08".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Did you see the new design?".to_string(),
            time: "17:10".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Yes, it looks amazing!".to_string(),
            time: "17:11".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Can't wait to try it out.".to_string(),
            time: "17:12".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Same here!".to_string(),
            time: "17:13".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Lunch break?".to_string(),
            time: "12:00".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Sure, let's go.".to_string(),
            time: "12:01".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "What do you want to eat?".to_string(),
            time: "12:02".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Sushi?".to_string(),
            time: "12:03".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Perfect!".to_string(),
            time: "12:04".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "See you in 10 minutes.".to_string(),
            time: "12:05".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "👍".to_string(),
            time: "12:06".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Did you get the tickets?".to_string(),
            time: "18:00".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "Yes, got them online.".to_string(),
            time: "18:01".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "Awesome, thanks!".to_string(),
            time: "18:02".to_string(),
            sender: ChatSender::Sent,
        },
        ChatMessage {
            text: "You're welcome!".to_string(),
            time: "18:03".to_string(),
            sender: ChatSender::Received("Anna".to_string()),
        },
        ChatMessage {
            text: "See you at the concert!".to_string(),
            time: "18:04".to_string(),
            sender: ChatSender::Sent,
        },
    ]
    .into();
    let messages = RwSignal::new(example_messages);

    let on_fetch_more = Callback::new(move |_| {
        // Simulate loading older messages (replace with real fetch logic)
        let more_messages = vec![
            ChatMessage {
                text: "Hey, did you check out the new cafe?".to_string(),
                time: "14:20".to_string(),
                sender: ChatSender::Sent,
            },
            ChatMessage {
                text: "Not yet, but I heard it's great!".to_string(),
                time: "14:21".to_string(),
                sender: ChatSender::Received("Anna".to_string()),
            },
            ChatMessage {
                text: "We should go sometime.".to_string(),
                time: "14:22".to_string(),
                sender: ChatSender::Sent,
            },
            ChatMessage {
                text: "Absolutely! Maybe this weekend?".to_string(),
                time: "14:23".to_string(),
                sender: ChatSender::Received("Anna".to_string()),
            },
            ChatMessage {
                text: "Sounds perfect.".to_string(),
                time: "14:24".to_string(),
                sender: ChatSender::Sent,
            },
            ChatMessage {
                text: "Let me know what time works for you.".to_string(),
                time: "14:25".to_string(),
                sender: ChatSender::Received("Anna".to_string()),
            },
            ChatMessage {
                text: "Will do!".to_string(),
                time: "14:26".to_string(),
                sender: ChatSender::Sent,
            },
            ChatMessage {
                text: "By the way, did you see the movie trailer?".to_string(),
                time: "13:50".to_string(),
                sender: ChatSender::Sent,
            },
            ChatMessage {
                text: "Yes, it looks awesome! Can't wait.".to_string(),
                time: "13:51".to_string(),
                sender: ChatSender::Received("Anna".to_string()),
            },
        ];
        messages.update(|msgs| {
            for msg in more_messages.into_iter().rev() {
                msgs.push_front(msg);
            }
        });
    });

    view! {
        <Groups>
            <Group
                name="Project Phoenix".to_string()
                last_message="Sophia: Final designs ready!".to_string()
                picture="https://i.pravatar.cc/100?img=12".to_string()
            />
            <Group
                name="Marketing Sprint".to_string()
                last_message="Liam: Updated docs uploaded.".to_string()
                picture="https://i.pravatar.cc/100?img=15".to_string()
            />
            <Group
                name="UI Design Team".to_string()
                last_message="Ethan: Reviewing the new layout...".to_string()
                picture="https://i.pravatar.cc/100?img=9".to_string()
            />
        </Groups>
        <Chat messages=messages on_fetch_more/>
    }
}
