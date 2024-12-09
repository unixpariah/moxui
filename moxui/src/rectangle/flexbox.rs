use calc_units::Units;

pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

pub enum JustifyContent {
    Start,
    Center,
    End,
    FlexStart,
    FlexEnd,
    Left,
    Right,
    Normal,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
    Safe(Box<JustifyContent>),
    Unsafe(Box<JustifyContent>),
}

pub enum AlignItems {
    Normal,
    Stretch,
    Center,
    Start,
    End,
    FlexStart,
    SelfStart,
    SelfEnd,
    AnchorCenter,
    Baseline,
    First(Box<AlignItems>),
    Last(Box<AlignItems>),
    Safe(Box<AlignItems>),
    Unsafe(Box<AlignItems>),
}

pub enum AlignContent {
    Normal,
    Stretch,
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Baseline,
    First(Box<AlignContent>),
    Last(Box<AlignContent>),
    Safe(Box<AlignContent>),
    Unsafe(Box<AlignContent>),
}

pub struct Order(pub i32);

pub struct FlexGrow(pub i32);

pub struct FlexShrink(pub i32);

pub enum FlexBasis {
    Auto,
    Content,
    Length(Units),
}

pub enum AlignSelf {
    Auto,
    Normal,
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
    AnchorCenter,
    Baseline,
    Stretch,
    First(Box<AlignSelf>),
    Last(Box<AlignSelf>),
    Safe(Box<AlignSelf>),
    Unsafe(Box<AlignSelf>),
}
