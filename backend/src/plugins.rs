use rquickjs::{Context, Ctx, FromJs, Function, IntoJs, Runtime, Value};
use std::path::Path;
use tokio::fs;
use crate::modules::{js_sdk, open_url};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Either<TA, TB> {
    A(TA),
    B(TB),
}

impl<TA, TB> From<TA> for Either<TA, TB> {
    fn from(value: TA) -> Self {
        Self::A(value)
    }
}

impl<'js, TA, TB> FromJs<'js> for Either<TA, TB>
where
    TA: FromJs<'js>,
    TB: FromJs<'js>,
{
    fn from_js(ctx: &Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        TA::from_js(ctx, value.clone())
            .map(Either::A)
            .or_else(|_| TB::from_js(ctx, value).map(Either::B))
    }
}

impl<'js, TA, TB> IntoJs<'js> for Either<TA, TB>
where
    TA: IntoJs<'js>,
    TB: IntoJs<'js>,
{
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        match self {
            Either::A(a) => a.into_js(ctx),
            Either::B(b) => b.into_js(ctx),
        }
    }
}
pub struct PluginsRuntime {
    pub context: Context,
}
fn print(s: String) {
    println!("{s}");
}
impl PluginsRuntime {
    pub(crate) fn new(runtime: &Runtime) -> PluginsRuntime {
        log::debug!("Initializing plugins runtime");
        let ctx = Context::full(runtime).unwrap();
        log::debug!("Initializing modules");
        ctx.with(|ctx| {
            let global = ctx.globals();
            
            global.set("openUrl", Function::new(ctx.clone(), open_url)).unwrap();
            global
                .set(
                    "__print",
                    Function::new(ctx.clone(), print)
                        .unwrap()
                        .with_name("__print")
                        .unwrap(),
                )
                .unwrap();
            ctx.eval::<(), _>(
                r#"
globalThis.console = {
  log(...v) {
    globalThis.__print(`${v.join(" ")}`)
  }
}
"#,
            )
            .unwrap();
        });
        log::debug!("Initialized modules");
        Self { context: ctx }
    }
    pub async fn load(&self, path: &Path) -> rquickjs::Result<()> {
        log::debug!("Loading {:?}", path);
        let content = fs::read_to_string(path).await?;
        self.context.with(|ctx| ctx.eval(content))?;
        log::debug!("Loaded {:?}", path);
        Ok(())
    }
}
