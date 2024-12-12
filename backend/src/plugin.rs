use reqwest::{Method, Request, Url};
use quickjs_runtime

pub struct PluginsRuntime {
    context: Context,
}

async fn fetchText(url: String, options: Option<FetchOptions>) -> Result<OwnedJsValue, String> {
    Request::new(
        options.map(|o| o.method).unwrap_or(Method::GET),
        Url::parse(&url).unwrap(),
    );
    reqwest::get(url)
}
impl PluginsRuntime {
    fn new() -> PluginsRuntime {
        let ctx = Context::new(None).unwrap();

        ctx.add_callback("fetch", fetchText).unwrap();
        ctx.add_callback("openUrl", |v: String| open::that(v).unwrap())
            .unwrap();
        Self { context: ctx }
    }
}
