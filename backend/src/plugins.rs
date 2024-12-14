use rquickjs::{Context, Ctx, FromJs, IntoJs, Runtime, Value};

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
    context: Context,
}
impl PluginsRuntime {
    pub(crate) fn new(runtime: &Runtime) -> PluginsRuntime {
        let ctx = Context::builder().build(runtime).unwrap();
        ctx.with(|ctx| {
            super::modules::setup(ctx);
        });
        Self { context: ctx }
    }
}
