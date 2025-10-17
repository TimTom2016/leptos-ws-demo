use api::get_pow;
use api::server_fn::login::Login;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_captcha::Captcha;
use leptos_router::hooks::use_query_map;

use crate::components::card::{Card, CardBody, CardHeader};
use crate::components::{
    button::{Button, ButtonVariant},
    input::{HiddenField, InputField},
};
use crate::contexts::account_context::AccountContext;

leptos_styling::style_sheet!(login_styles, "src/pages/login.module.scss", "login");

#[component]
pub fn LoginPage() -> impl IntoView {
    let action = ServerAction::<Login>::new();
    let is_pending = RwSignal::new(None);
    let account = use_context::<AccountContext>().expect("AccountContext not found");
    let on_submit = move |ev: SubmitEvent| {
        if let Ok(mut data) = Login::from_event(&ev) {
            ev.prevent_default();

            // Currently, the Captcha validation is running thread local.
            // This means a too high difficulty will block the thread.
            // The default of 20 is reasonable for a release build, but
            // way too high for development.
            //
            // The validation might me improved in the future by using
            // a web worker for this purpose, but this is not yet implemented.
            leptos_captcha::pow_dispatch(get_pow, is_pending, move |pow| {
                if let Ok(pow) = pow {
                    data.pow = pow;
                }

                action.dispatch(data);
                set_timeout(
                    move || {
                        account.refresh();
                    },
                    std::time::Duration::from_millis(500),
                );
            })
        }
    };
    let query = use_query_map();
    let next = Signal::derive(move || {
        query.with(|q| {
            q.get("next")
                .map(|x| x.to_string())
                .unwrap_or("/".to_string())
        })
    });

    view! {
        <main class="auth-area">
            <div class=login_styles::LOGIN_CONTAINER>
                <Card>
                    <CardHeader>
                        <h1>Login</h1>
                    </CardHeader>
                    <CardBody>
                        <form on:submit=on_submit class=login_styles::LOGIN_FORM>
                            <InputField
                                name="username"
                                id="username"
                                label="Username"
                                maxlength=32
                            />

                            <InputField
                                name="password"
                                id="password"
                                label="Password"
                                input_type="password"
                            />

                            <HiddenField name="next" value=next.get() />

                            <div class=login_styles::CAPTCHA_CONTAINER class:d-none=true>
                                <Captcha is_pending=is_pending />
                            </div>

                            <div class=login_styles::BUTTON_CONTAINER>
                                <Button sizing=crate::components::button::Sizing::Big variant=ButtonVariant::Primary>
                                    "Login"
                                </Button>
                            </div>
                        </form>
                    </CardBody>
                </Card>
            </div>
        </main>
    }
}
