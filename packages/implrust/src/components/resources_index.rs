use crate::*;

#[component]
pub(crate) fn FeaturedWidgets() -> Element {
    let widgets = vec![
        (
            "ImplRust Projects".to_string(),
            "Projects created by ImplRust.".to_string(),
            "https://github.com/implferris".to_string(),
        ),
        (
            "Free Online Books".to_string(),
            "A curated list of free online books about Rust, organized by category.".to_string(),
            "/resources/free-rust-books".to_string(),
        ),
        (
            "Youtube Channels".to_string(),
            "List of popular and underrated youtube channels that covers Rust.".to_string(),
            "/resources/youtube-channels".to_string()
        ),
        (
            "Curated List".to_string(),
            "Browse a curated selection of high-quality resources for embedded Rust programming, ideal for all developers.".to_string(),
            "/resources/curated-list".to_string(),
        ),
        (
            "Blog Posts Collection".to_string(),
            "Access insightful blog posts on various Rust topics, featuring tutorials.".to_string(),
            "/resources/general#blog-posts".to_string(),
        ),
        (
            "Rust Cheat Sheet".to_string(),
            "Utilize a handy cheat sheet summarizing key Rust concepts and syntax for quick reference.".to_string(),
            "/resources/general#rust-language-cheat-sheet".to_string()
        ),
    ];

    rsx! {
        div { class: "flex items-center justify-center min-h-screen",
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6 max-w-5xl w-full",
                for (title, description, link) in widgets.iter() {
                    Widget { title: title.clone(), description: description.clone(), link: link.clone() }
                }
            }
        }
    }
}

#[component]
pub(crate) fn Widget(title: String, description: String, link: String) -> Element {
    rsx! {
        a{
            href: "{link}", ">>"
            div { class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-[8px_8px_0px_0px_rgba(249,115,22,0.8)] w-72 h-48",
                h2 { class: "text-xl font-semibold text-gray-900 dark:text-white", "{title}" }
                p { class: "text-gray-600 dark:text-gray-300 mt-2", "{description}" }
            }
        }
    }
}
