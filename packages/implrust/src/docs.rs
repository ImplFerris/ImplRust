use crate::Route;
use dioxus::prelude::*;
use mdbook_shared::MdBook;
use std::hash::Hash;

pub use dioxus_site_router::docs::*;

pub enum CurrentDocsVersion {
    V06(router_learn::BookRoute),
}

impl CurrentDocsVersion {
    pub fn full_version(&self) -> &'static str {
        match self {
            CurrentDocsVersion::V06(_) => router_learn::BookRoute::full_version(),
        }
    }
}

pub fn use_try_current_docs_version() -> Option<CurrentDocsVersion> {
    let route = use_route();
    match route {
        Route::LearnMd { child } => Some(CurrentDocsVersion::V06(child)),
        _ => None,
    }
}

pub trait AnyBookRoute: Routable + PartialEq + Hash + Eq + Clone + Copy {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section];
    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self>;
    fn global_route(&self) -> crate::Route;
    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId;
    fn book() -> &'static MdBook<Self>;
    fn use_current() -> Option<Self>;
    fn use_route() -> Self {
        Self::use_current().expect("current route to be the same as the route")
    }
    fn short_version() -> &'static str;
    fn full_version() -> &'static str;
    fn index() -> Self;
}

impl AnyBookRoute for router_blog::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::BlogPost { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_blog::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::BlogPost { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "blog"
    }
    fn full_version() -> &'static str {
        "Blog"
    }
    fn index() -> Self {
        todo!()
    }
}

impl AnyBookRoute for router_learn::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::LearnMd { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_learn::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::LearnMd { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "learn"
    }
    fn full_version() -> &'static str {
        "Learn"
    }
    fn index() -> Self {
        Self::Index {
            section: Default::default(),
        }
    }
}

impl AnyBookRoute for router_resources::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::ResourcesRoute { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_resources::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::ResourcesRoute { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "resources"
    }
    fn full_version() -> &'static str {
        "Resources"
    }
    fn index() -> Self {
        Self::Index {
            section: Default::default(),
        }
    }
}
