use reqwest::{Method, Request, Url};
use rquickjs::class::Trace;
use rquickjs::{AsyncContext, AsyncRuntime, IntoJs, JsLifetime, Module};
use std::collections::HashMap;
use std::str::FromStr;

#[rquickjs::module(rename_vars = "camelCase", rename = "@gami/sdk")]
mod stuff {
    #[rquickjs::module(rename_vars = "camelCase")]
    mod utils {
        #[rquickjs::function]
        fn open_url(url: String) -> Result<(), rquickjs::Error> {
            open::that(url).map_err(|e| rquickjs::Error::new_from_js_message("", "", e.to_string()))
        }
    }
}
pub struct PluginsRuntime {
    context: AsyncContext,
}

#[rquickjs::class(rename_all = "camelCase")]
#[derive(Debug, JsLifetime, Clone, Trace)]
struct FetchOptions {
    pub body: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}

#[rquickjs::function]
async fn fetchText(url: String, options: Option<FetchOptions>) -> Result<String, rquickjs::Error> {
    Request::new(
        options
            .and_then(|o| Method::from_str(&o.method).ok())
            .unwrap_or(Method::GET),
        Url::parse(&url).unwrap(),
    );
    let res = reqwest::get(url)
        .await
        .map_err(|e| rquickjs::Error::Unknown)?;
    Ok(res
        .text()
        .await
        .map_err(|e| rquickjs::Error::new_from_js_message("", "", e.to_string()))?)
}
impl PluginsRuntime {
    async fn new(runtime: &AsyncRuntime) -> PluginsRuntime {
        let ctx = AsyncContext::builder().build_async(runtime).await.unwrap();
        ctx.with(|ctx| {
            Module::declare_def::<js_utils, _>(ctx.clone(), "utils").unwrap();
        })
        .await;
        Self { context: ctx }
    }
}
