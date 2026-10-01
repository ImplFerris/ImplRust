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
            "/resources/free-rust-books/".to_string(),
        ),
        (
            "Youtube Channels".to_string(),
            "List of popular and underrated youtube channels that covers Rust.".to_string(),
            "/resources/youtube-channels/".to_string()
        ),
        (
            "Curated List".to_string(),
            "Browse a curated selection of high-quality resources for embedded Rust programming, ideal for all developers.".to_string(),
            "/resources/curated-list/".to_string(),
        ),
        (
            "Blog Posts Collection".to_string(),
            "Access insightful blog posts on various Rust topics, featuring tutorials.".to_string(),
            "/resources/general/#blog-posts".to_string(),
        ),
        (
            "Rust Cheat Sheet".to_string(),
            "Utilize a handy cheat sheet summarizing key Rust concepts and syntax for quick reference.".to_string(),
            "/resources/general#rust-language-cheat-sheet".to_string()
        ),
    ];

    rsx! {
        main { class: "resources-page",
            div { class: "resources-shell",
                div { class: "resources-intro",
                    span { class: "resources-eyebrow", "RUST RESOURCES" }
                    h1 { "Learn Rust, your way." }
                    p { "A hand-picked collection of projects, books, videos, and practical references for every stage of your Rust journey." }
                }
                div { class: "resources-grid",
                for (title, description, link) in widgets.iter() {
                    Widget { title: title.clone(), description: description.clone(), link: link.clone() }
                }
                }
            }
        }
    }
}

#[component]
pub(crate) fn Widget(title: String, description: String, link: String) -> Element {
    rsx! {
        a { class: "resource-card", href: "{link}",
            div { class: "resource-card-content",
                div { class: "resource-card-heading",
                    h2 { "{title}" }
                }
                p { "{description}" }
                span { class: "resource-card-arrow", aria_hidden: "true", "→" }
            }
        }
    }
}
