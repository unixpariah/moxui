#[derive(Clone, Debug, PartialEq)]
pub enum Units {
    Px(f32),
    Perc(f32),
    Vw(f32),
    Vh(f32),
    Vmin(f32),
    Vmax(f32),
    In(f32),
    Mm(f32),
    Pt(f32),
    Pc(f32),
    Em(f32),
    Rem(f32),
    Calc(Box<CalcExpr>),
    Auto,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CalcExpr {
    Value(Units),
    Add(Box<CalcExpr>, Box<CalcExpr>),
    Sub(Box<CalcExpr>, Box<CalcExpr>),
    Mul(Box<CalcExpr>, Box<CalcExpr>),
    Div(Box<CalcExpr>, Box<CalcExpr>),
    Paren(Box<CalcExpr>),
}

impl CalcExpr {
    pub fn evaluate(&self, context: &Context) -> f32 {
        match self {
            CalcExpr::Value(unit) => unit.to_px(context),
            CalcExpr::Add(left, right) => left.evaluate(context) + right.evaluate(context),
            CalcExpr::Sub(left, right) => left.evaluate(context) - right.evaluate(context),
            CalcExpr::Mul(expr, scalar) => expr.evaluate(context) * scalar.evaluate(context),
            CalcExpr::Div(expr, scalar) => expr.evaluate(context) / scalar.evaluate(context),
            CalcExpr::Paren(expr) => expr.evaluate(context),
        }
    }
}

impl Units {
    pub fn to_px(&self, context: &Context) -> f32 {
        match self {
            Self::Px(num) => *num,
            Self::Perc(num) => (*num / 100.0) * context.reference_size,
            Self::Vw(num) => *num * context.viewport.0 / 100.0,
            Self::Vh(num) => *num * context.viewport.1 / 100.0,
            Self::Vmin(num) => *num * context.viewport.0.min(context.viewport.1) / 100.0,
            Self::Vmax(num) => *num * context.viewport.0.max(context.viewport.1) / 100.0,
            Self::In(num) => *num * context.dpi,
            Self::Mm(num) => *num / 25.4 * context.dpi,
            Self::Pt(num) => *num / 72.0 * context.dpi,
            Self::Pc(num) => *num * 12.0 / 72.0 * context.dpi,
            Self::Em(num) => *num * context.parent_font_size,
            Self::Rem(num) => *num * context.root_font_size,
            Self::Calc(expr) => expr.evaluate(context),
            Self::Auto => context.auto,
        }
    }

    pub fn is_auto(&self) -> bool {
        *self == Self::Auto
    }
}

pub struct Context {
    pub root_font_size: f32,
    pub reference_size: f32,
    pub parent_font_size: f32,
    pub viewport: (f32, f32),
    pub auto: f32,
    pub dpi: f32,
}
