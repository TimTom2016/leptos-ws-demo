use leptos::prelude::*;
use leptos_use::use_media_query;

#[derive(Clone, Debug, Copy)]
pub struct MobileDetection(pub Signal<bool>);
/// Provides a context for mobile detection, allowing components to access whether the user is on a mobile device.
#[component]
pub fn DetectMobile(children: Children) -> impl IntoView {
    let is_not_mobile = use_media_query("(min-width: 480px)");
    let is_mobile = Signal::derive(move || !is_not_mobile.get());
    Effect::new(move || {
        // Log the mobile detection status
        log::debug!("Mobile detection status: {}", is_mobile.get());
    });
    let mobile_detection = MobileDetection(is_mobile);
    provide_context(mobile_detection);
    view! {
        {children()}
    }
}

#[component]
pub fn ShowMobile(children: ChildrenFn, #[prop(optional, into)] fallback: ViewFn) -> impl IntoView {
    let mobile_detection = use_context::<MobileDetection>()
        .expect("ShowMobile must be used within DetectMobile context");
    let signal = mobile_detection.0;
    view! {
        <Show when=move || signal.get() fallback=fallback>
            {children()}
        </Show>
    }
}
