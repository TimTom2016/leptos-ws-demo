use api::server_fn::settings::{Account, get_account};
use leptos::{context::Provider, prelude::*};
#[derive(Clone, Debug, Copy)]
pub struct AccountContext(Resource<Option<Account>>);

impl AccountContext {
    pub fn new() -> Self {
        let resource = Resource::new(
            || (),
            |_| async {
                let user = get_account()
                    .await
                    .inspect_err(|e| log::error!("Could not load Account: {e:?}"))
                    .ok();
                user
            },
        );
        Self(resource)
    }
    #[inline]
    pub fn logged_in(&self) -> bool {
        self.0
            .read()
            .as_ref()
            .flatten()
            .is_some_and(|v| v.is_logged_in())
    }
    #[inline]
    pub fn user(&self) -> Option<Account> {
        self.0.get().flatten()
    }

    #[inline]
    pub fn user_untracked(&self) -> Option<Account> {
        self.0.get_untracked().flatten()
    }
    pub fn refresh(&self) {
        self.0.refetch();
    }
}

#[component]
pub fn AccountProvider<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <Provider value=AccountContext::new()>
            {(children.into_inner())()}
        </Provider>
    }
}

#[component]
pub fn LoggedIn(
    children: ChildrenFn,
    /// A closure that returns what gets rendered if the when statement is false. By default this is the empty view.
    #[prop(optional, into)]
    fallback: ViewFn,
) -> impl IntoView {
    let context = use_context::<AccountContext>().expect("AccountContext not found");
    view! {
        <Show when=move || context.logged_in() fallback=fallback>
            {children()}
        </Show>
    }
}
