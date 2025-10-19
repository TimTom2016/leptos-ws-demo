use std::collections::VecDeque;

use crate::components::{
    chat::{Chat, SelectGroup},
    groups::{Group, Groups},
};
use leptos::{either::Either, prelude::*};
use leptos_router::{hooks::use_query, params::Params};

#[derive(Params, PartialEq)]
struct HomeQuery {
    group: Option<String>,
}

#[component]
pub fn ChatPage() -> impl IntoView {
    let reload_groups = Trigger::new();
    let query = use_query::<HomeQuery>();
    let group_id = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|query| query.group.clone())
    };
    let groups = Resource::new(
        || (),
        |_| async { api::server_fn::groups::get_groups().await },
    );

    Effect::new(move |previous| {
        reload_groups.track();
        if let Some(prev) = previous {
            groups.refetch();
        }
    });

    view! {
        <Groups reload_groups>
            <Suspense>
                {move || {
                    groups.and_then(|v| {
                        let groups = v.to_owned();
                        view!{
                            <For each=move || groups.clone() key=move |group| group.id.clone()
                            let:group>
                                <Group
                                    id=group.id.clone()
                                    name=group.name.clone()
                                    last_message=group.last_message.clone()
                                    picture=group.avatar_url.clone()
                                    join_code=group.join_code.clone()
                                />
                            </For>
                        }
                    })
                }}

            </Suspense>
        </Groups>
        {move || match group_id() {
            Some(id) => Either::Left(view!{
            <Chat group_id=id/>
            }),
            None => Either::Right(SelectGroup)
        }}
    }
}
