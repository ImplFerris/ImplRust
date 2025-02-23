use dioxus::prelude::*;

pub fn Footer() -> Element {
    rsx! {
        footer { class: "text-gray-700 dark:text-gray-400 w-full mx-auto max-w-screen-xl px-2",
            div { class: "text-gray-400 text-sm text-center sm:text-left pb-2 mx-auto",
                "© 2025 ImplRust - This is unofficial site (It is in no way endorsed nor affiliated with the Rust foundation)."
            }
        }
    }
}
