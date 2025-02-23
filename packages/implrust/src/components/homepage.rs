use crate::*;

pub(crate) mod hero;

#[component]
pub(crate) fn Homepage() -> Element {
    rsx! {
        div { class: "w-full",
            hero::Hero {}
        }
    }
}
