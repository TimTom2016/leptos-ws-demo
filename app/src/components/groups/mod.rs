use leptos::{either::EitherOf3, prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::components::{
    button::Button,
    dialog::{Dialog, DialogBody, DialogHeader},
    input::InputField,
    multi_step::{MultiStep, Step},
    spinner::Spinner,
};

leptos_styling::style_sheet!(
    groups_styles,
    "src/components/groups/groups.module.scss",
    "groups"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddType {
    Create,
    Join,
}

#[component]
pub fn Groups<T>(children: TypedChildren<T>, reload_groups: Trigger) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    let open_add = RwSignal::new(false);
    let reset = Trigger::new();
    let selected_add_type = RwSignal::new(None);
    let name = RwSignal::new(String::new());
    let group_picture = RwSignal::new(String::new());
    let join_code = RwSignal::new(String::new());
    let finished_api_request = RwSignal::new(false);
    Effect::new(move |_| {
        if open_add.get() {
            name.set(String::new());
            group_picture.set(String::new());
            join_code.set(String::new());
            finished_api_request.set(false);
            reset.notify();
        }
    });
    Effect::new(move |_| {
        if open_add.get() {
            name.set(String::new());
            group_picture.set(String::new());
            join_code.set(String::new());
            finished_api_request.set(false);
        }
    });
    view! {
        <Dialog open=open_add on_outside_click=Callback::new(move |_| {
            log::info!("Dialog closed");
            open_add.set(false);
        })>
            <DialogHeader>
                <h2>Add Group</h2>
            </DialogHeader>
            <DialogBody>
                <MultiStep
                    show_class="fade-in-500"
                    hide_class="fade-out-250"
                    hide_delay=std::time::Duration::from_millis(250)
                    reset
                    >
                        <Step slot:steps children={move |next: Callback<()>| {
                            view!{
                                <div class=groups_styles::DECIDE_ADD>
                                    <button on:click=move |_| {
                                        selected_add_type.set(Some(AddType::Create));
                                        next.run(());
                                    }>Create<Icon icon=icondata::IoAddOutline width="7.5rem" height="7.5rem"/></button>
                                    <button on:click=move |_| {
                                        selected_add_type.set(Some(AddType::Join));
                                        next.run(());
                                    }>Join<Icon icon=icondata::LuUserPlus width="7.5rem" height="7.5rem"/></button>
                                </div>
                            }
                        }}/>
                        <Step slot:steps children={move |next: Callback<()>| {
                            {match selected_add_type.get() {
                                Some(AddType::Create) => EitherOf3::A(view! {
                                    <div>
                                        <h3>"Create Group"</h3>
                                        <InputField
                                            name="name"
                                            id="name"
                                            maxlength=128
                                            label="Name"
                                            prop:value=name
                                            on:input=move |e| {
                                                name.set(event_target_value(&e));
                                            }
                                        />
                                        <InputField
                                            name="Group Picture"
                                            id="url"
                                            maxlength=128
                                            label="Group Picture"
                                            prop:value=group_picture
                                            on:input=move |e| {
                                                group_picture.set(event_target_value(&e));
                                            }
                                        />
                                        <Button
                                            variant=crate::components::button::ButtonVariant::Primary
                                            center=true
                                            on:click=move |_| {
                                                spawn_local(async move {
                                                    let result = api::server_fn::groups::create_group(
                                                        name.get().clone(),
                                                        group_picture.get().clone(),
                                                    ).await;
                                                    if let Err(err) = result {
                                                        log::error!("Failed to create Group: {err:?}");
                                                        return;
                                                    }
                                                    finished_api_request.set(true);
                                                });
                                                next.run(());
                                            }>
                                            "Create"
                                        </Button>
                                    </div>
                                }),
                                Some(AddType::Join) => EitherOf3::B(view!{
                                    <div>
                                        <h3>"Join Group"</h3>
                                        <InputField
                                            name="join_code"
                                            id="join_code"
                                            maxlength=128
                                            label="Join Code"
                                            prop:value=join_code
                                            on:input=move |e| {
                                                join_code.set(event_target_value(&e));
                                            }
                                        />
                                        <Button
                                            variant=crate::components::button::ButtonVariant::Primary
                                            center=true
                                            on:click=move |_| {
                                                let Ok(join_code) = join_code.get().parse().inspect_err(|err| {
                                                    log::error!("Failed to parse join code: {err}");
                                                }) else {
                                                    return;
                                                };
                                                spawn_local(async move {
                                                    let result = api::server_fn::groups::join_group(
                                                        join_code
                                                    ).await;
                                                    if let Err(err) = result {
                                                        log::error!("Failed to create Group: {err:?}");
                                                        return;
                                                    }
                                                    finished_api_request.set(true);
                                                });
                                                next.run(());
                                            }>
                                            "Join"
                                        </Button>
                                    </div>
                                }),
                                None => EitherOf3::C(view! { <p>"An Error Ocurred."</p> }),
                            }}
                        }}/>
                        <Step slot:steps children={move |next: Callback<()>| {
                            Effect::new(move || {
                                if finished_api_request.get() {
                                    reload_groups.notify();
                                    open_add.set(false);
                                    reset.notify();
                                }
                            });
                            view!{
                                <div>
                                    <h3>"Joining Classroom"</h3>
                                    <Spinner size=crate::components::spinner::SpinnerSize::Large/>
                                </div>
                            }
                        }}/>

                    </MultiStep>
            </DialogBody>
        </Dialog>
        <div class=groups_styles::GROUPS>
            <h2>"Groups"</h2>
            <div class=groups_styles::GROUP_LIST>
                {(children.into_inner())()}
                <Button variant=crate::components::button::ButtonVariant::Tertiary center=true {..} on:click=move |_| open_add.set(true)>"+"</Button>
            </div>
        </div>
    }
}

#[component]
pub fn Group(
    id: String,
    name: String,
    last_message: String,
    picture: String,
    join_code: String,
) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <A href=format!("?group={id}") {..} class=groups_styles::GROUP>
            <img src={picture} alt={name.clone()} />
            <div class=groups_styles::DETAILS>
                <h3>{name}</h3>
                <p>{last_message}</p>
            </div>
            <Button variant=crate::components::button::ButtonVariant::Secondary center=true {..} on:click=move |_| open.set(true)>"Code"</Button>


        </A>
        <Dialog open=open on_outside_click=Callback::new(move |_| {
            open.set(false);
        })>
            <DialogHeader>
                <h2>Join Code</h2>
            </DialogHeader>
            <DialogBody>
                <h1 class=groups_styles::JOIN_CODE>{join_code}</h1>
            </DialogBody>
        </Dialog>
    }
}
