pub fn generate_search_index() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::{static_dir, Route};

        std::env::set_var("CARGO_MANIFEST_DIR", static_dir().join("assets"));
        let version_filter: [(&str, fn(&Route) -> bool); 1] =
            [("0_6", |route| matches!(route, Route::LearnMd { .. }))];
        for (version, filter) in version_filter {
            dioxus_search::SearchIndex::<Route>::create(
                format!("searchable_{version}"),
                dioxus_search::BaseDirectoryMapping::new(static_dir()).map(|route| {
                    filter(&route).then(|| {
                        let route = route.to_string();
                        let (route, _) = route.split_once('#').unwrap_or((&route, ""));
                        let (route, _) = route.split_once('?').unwrap_or((&route, ""));
                        std::path::PathBuf::from(route).join("index.html")
                    })
                }),
            );
        }
    }
}
