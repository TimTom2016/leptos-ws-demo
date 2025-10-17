use std::{sync::Arc, time::Duration};

use leptos::prelude::*;

#[derive(Clone)]
pub struct ViewFnWithInput<F>(Arc<dyn Fn(F) -> AnyView + Send + Sync + 'static>);

impl<F, C, I> From<F> for ViewFnWithInput<I>
where
    F: Fn(I) -> C + Send + Sync + 'static,

    C: RenderHtml + Send + 'static,
{
    fn from(value: F) -> Self {
        Self(Arc::new(move |data: I| value(data).into_any()))
    }
}

impl<I> ViewFnWithInput<I> {
    pub fn run(&self, value: I) -> AnyView {
        (self.0)(value)
    }
}

impl<I> Default for ViewFnWithInput<I> {
    fn default() -> Self {
        Self(Arc::new(|_: I| ().into_any()))
    }
}

#[derive(Clone)]
#[slot]
pub struct Step {
    #[prop(into)]
    pub children: ViewFnWithInput<Callback<()>>,
    #[prop(into, optional)]
    pub skip_if: Option<Signal<bool>>,
}

#[component]
pub fn MultiStep(
    steps: Vec<Step>,
    hide_class: &'static str,
    show_class: &'static str,
    hide_delay: Duration,
    reset: Trigger,
) -> impl IntoView {
    let length = steps.len();
    let current = RwSignal::new(0);
    let hide = RwSignal::new(false);
    let next = Callback::new(move |_| {
        if *hide.read_untracked() {
            return;
        }
        *hide.write() = true;
        set_timeout(
            move || {
                *hide.write() = false;
                current.update(move |c| {
                    if length - 1 > *c {
                        *c += 1;
                    }
                });
            },
            Duration::from_millis(hide_delay.as_millis() as u64 + 10),
        );
    });
    Effect::new(move |prev| {
        reset.track();
        current.set(0);
        hide.set(false);
    });
    view! {
        {move || {
            let step = steps[current.get()].clone();
            let children = step.children.clone();
            if let Some(skip_if) = step.skip_if {
                if skip_if.get_untracked() {
                    next.run(());
                }
            }
            view!{
                <AnimatedShow when=Signal::derive(move || !hide.get()) hide_class show_class hide_delay>
                    {children.run(next)}
                </AnimatedShow>
            }
        }}
    }
}
