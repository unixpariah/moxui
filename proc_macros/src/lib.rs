use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, BinOp, Expr, Lit, UnOp};

#[proc_macro]
pub fn calc(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let expr = build_calc_expr(&expr);
    let result = quote! {
        {
            use calc_units::{CalcExpr, Units};
            Units::Calc(Box::new(#expr))
        }
    };
    result.into()
}

fn build_calc_expr(expr: &Expr) -> TokenStream2 {
    match expr {
        Expr::Binary(op) => {
            let left = build_calc_expr(&op.left);
            let right = build_calc_expr(&op.right);
            match op.op {
                BinOp::Add(_) => quote! { CalcExpr::Add(Box::new(#left), Box::new(#right)) },
                BinOp::Sub(_) => quote! { CalcExpr::Sub(Box::new(#left), Box::new(#right)) },
                BinOp::Mul(_) => quote! { CalcExpr::Mul(Box::new(#left), Box::new(#right)) },
                BinOp::Div(_) => quote! { CalcExpr::Div(Box::new(#left), Box::new(#right)) },
                _ => panic!("Unsupported binary operation"),
            }
        }
        Expr::Unary(op) => {
            let expr = build_calc_expr(&op.expr);
            match op.op {
                UnOp::Neg(_) => {
                    quote! { CalcExpr::Sub(Box::new(CalcExpr::Value(Units::Px(0.0))), Box::new(#expr)) }
                }
                _ => panic!("Unsupported unary operation"),
            }
        }
        Expr::Lit(lit) => match &lit.lit {
            Lit::Int(int_lit) => {
                if let Some(value) = int_lit.to_string().strip_suffix("px") {
                    let value = value.parse::<f32>().unwrap();
                    quote! { CalcExpr::Value(Units::Px(#value))}
                } else if let Some(value) = int_lit.to_string().strip_suffix("vw") {
                    let value = value.parse::<f32>().unwrap();
                    quote! { CalcExpr::Value(Units::Vw(#value))}
                } else if let Some(value) = int_lit.to_string().strip_suffix("vh") {
                    let value = value.parse::<f32>().unwrap();
                    quote! { CalcExpr::Value(Units::Vh(#value))}
                } else if let Some(value) = int_lit.to_string().strip_suffix("vmin") {
                    let value = value.parse::<f32>().unwrap();
                    quote! { CalcExpr::Value(Units::Vmin(#value))}
                } else if let Some(value) = int_lit.to_string().strip_suffix("vmax") {
                    let value = value.parse::<f32>().unwrap();
                    quote! { CalcExpr::Value(Units::Vmax(#value))}
                } else {
                    panic!("Unsupported suffix")
                }
            }
            Lit::Float(float_lit) => {
                let float_value = float_lit.base10_parse::<f32>().unwrap();
                quote! { CalcExpr::Value(Units::Px(#float_value)) }
            }
            Lit::Str(str_lit) => {
                if let Some(percentage_value) = str_lit.value().strip_suffix('%') {
                    let perc_value = percentage_value.parse::<f32>().unwrap();
                    quote! { CalcExpr::Value(Units::Perc(#perc_value)) }
                } else {
                    panic!("Unsupported literal type");
                }
            }
            _ => panic!("Unsupported literal type"),
        },
        Expr::Paren(paren) => {
            let paren = build_calc_expr(&paren.expr);
            quote! { CalcExpr::Paren(Box::new(#paren)) }
        }
        _ => panic!("Unsupported expression type"),
    }
}
