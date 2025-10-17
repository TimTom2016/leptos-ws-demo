use leptos::prelude::*;

#[component]
pub fn SvgInclude(
    #[prop(into)] src: String,
    #[prop(into,default=String::from("100px"))] width: String,
    #[prop(into,default=String::from("100px"))] height: String,
    #[prop(into, default=String::from("Alternative SVG"))] alt: String,
) -> impl IntoView {
    view! {
        <object data=src.clone() type="image/svg+xml" width=width.clone() height=height.clone()>
            <img src=src.clone() width=width.clone() height=height.clone() alt=alt/>
        </object>
    }
}
