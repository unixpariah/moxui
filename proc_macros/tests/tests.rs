use calc_macro::calc;

static CONTEXT: calc_units::Context = calc_units::Context {
    root_font_size: 16.0,
    viewport: (1920.0, 1080.0),
    parent_font_size: 16.0,
    reference_size: 200.0,
    auto: 0.0,
    dpi: 97.1,
};

use calc_units::Units;

fn evaluate_calc(units: Units) -> f32 {
    match units {
        Units::Calc(expr) => expr.evaluate(&CONTEXT),
        _ => panic!("Expected Units::Calc variant"),
    }
}

#[test]
fn test_cal_subtraction() {
    let result = calc!(30px - 10px);
    assert_eq!(evaluate_calc(result), 20.0);
}

#[test]
fn test_cal_multiplication() {
    let result = calc!(5px * 6px);
    assert_eq!(evaluate_calc(result), 30.0);
}

#[test]
fn test_cal_division() {
    let result = calc!(20px / 4px);
    assert_eq!(evaluate_calc(result), 5.0);
}

#[test]
fn test_cal_nested_expression() {
    let result = calc!((10px + 20px) * 2px);
    assert_eq!(evaluate_calc(result), 60.0);
}

#[test]
fn test_cal_negative_value() {
    let result = calc!(-10px);
    assert_eq!(evaluate_calc(result), -10.0);
}

#[test]
fn test_cal_percentage_addition() {
    let result = calc!("20%" + "30%");
    assert_eq!(evaluate_calc(result).round(), 100.0);
}

#[test]
fn test_cal_percentage_of_parent_size() {
    let result = calc!("50%");
    assert_eq!(evaluate_calc(result).round(), 100.0);
}

#[test]
fn test_cal_percentage_subtraction() {
    let result = calc!("60%" - "20%");
    assert_eq!(evaluate_calc(result).round(), 80.0);
}

#[test]
fn test_cal_percentage_multiplication() {
    let result = calc!("20%" * 2.0);
    assert_eq!(evaluate_calc(result).round(), 80.0);
}

#[test]
fn test_cal_percentage_division() {
    let result = calc!("50%" / 2.0);
    assert_eq!(evaluate_calc(result).round(), 50.0);
}

#[test]
fn test_cal_vh() {
    let result = calc!(50vh);
    assert_eq!(evaluate_calc(result), 540.0);
}

#[test]
fn test_cal_vw() {
    let result = calc!(25vw);
    assert_eq!(evaluate_calc(result), 480.0);
}

#[test]
fn test_cal_vmin() {
    let result = calc!(10vmin);
    assert_eq!(evaluate_calc(result), 108.0);
}

#[test]
fn test_cal_vmax() {
    let result = calc!(15vmax);
    assert_eq!(evaluate_calc(result), 288.0);
}

#[test]
fn test_cal_combined_vh_vw() {
    let result = calc!(10vh + 20vw);
    assert_eq!(evaluate_calc(result), 492.0);
}
