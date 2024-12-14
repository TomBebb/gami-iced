#[rquickjs::module(rename_vars = "camelCase", rename = "@gami/sdk")]
pub mod sdk {
    use crate::models::GameLibrary;
    #[rquickjs::module(rename_vars = "camelCase")]
    pub mod utils {
        #[rquickjs::function]
        pub fn open_url(url: String) -> Result<(), rquickjs::Error> {
            open::that(url).map_err(|e| rquickjs::Error::new_from_js_message("", "", e.to_string()))
        }
    }

    #[rquickjs::module(rename_vars = "camelCase")]
    mod http {
        use reqwest::{Method, Request, Url};
        use rquickjs::class::Trace;
        use rquickjs::JsLifetime;
        use std::collections::HashMap;
        use std::str::FromStr;
        #[rquickjs::class(rename_all = "camelCase")]
        #[derive(Debug, JsLifetime, Clone, Trace)]
        pub struct FetchOptions {
            pub body: String,
            pub method: String,
            pub headers: HashMap<String, String>,
        }

        #[rquickjs::function]
        async fn fetch_text(
            url: String,
            options: Option<FetchOptions>,
        ) -> Result<String, rquickjs::Error> {
            Request::new(
                options
                    .and_then(|o| Method::from_str(&o.method).ok())
                    .unwrap_or(Method::GET),
                Url::parse(&url).unwrap(),
            );
            let res = reqwest::get(url)
                .await
                .map_err(|_e| rquickjs::Error::Unknown)?;
            Ok(res
                .text()
                .await
                .map_err(|e| rquickjs::Error::new_from_js_message("", "", e.to_string()))?)
        }
    }

    #[rquickjs::function]
    fn register_library(lib: GameLibrary) {
        println!("Register library: {:?}", lib);
    }
}
