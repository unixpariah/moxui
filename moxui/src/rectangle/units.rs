pub enum Units {
    Px(f32),
    Perc(f32),
    Vw(f32),
    Vh(f32),
    Vmin(f32),
    Vmax(f32),
    Calc(Box<CalcExpr>),
}

pub enum CalcExpr {
    Value(Units),
    Add(Box<CalcExpr>, Box<CalcExpr>),
    Sub(Box<CalcExpr>, Box<CalcExpr>),
    Mul(Box<CalcExpr>, f32),
    Div(Box<CalcExpr>, f32),
}

impl CalcExpr {
    pub fn evaluate(&self, context: &Context) -> f32 {
        match self {
            CalcExpr::Value(unit) => unit.to_px(context),
            CalcExpr::Add(left, right) => left.evaluate(context) + right.evaluate(context),
            CalcExpr::Sub(left, right) => left.evaluate(context) - right.evaluate(context),
            CalcExpr::Mul(expr, scalar) => expr.evaluate(context) * scalar,
            CalcExpr::Div(expr, scalar) => expr.evaluate(context) / scalar,
        }
    }
}

impl Units {
    pub fn to_px(&self, context: &Context) -> f32 {
        match self {
            Self::Px(num) => *num,
            Self::Perc(num) => (*num / 100.0) * context.parent_size,
            Self::Vw(num) => *num * context.viewport.0 / 100.0,
            Self::Vh(num) => *num * context.viewport.1 / 100.0,
            Self::Vmin(num) => *num * context.viewport.0.min(context.viewport.1) as f32 / 100.0,
            Self::Vmax(num) => *num * context.viewport.0.max(context.viewport.1) / 100.0,
            Self::Calc(expr) => expr.evaluate(context),
        }
    }
}

pub struct Context {
    pub parent_size: f32,
    pub viewport: (f32, f32),
}
