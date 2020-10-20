//! Common types used in DOM representation.

/// Clone-on-write string.
pub type CowStr = std::borrow::Cow<'static, str>;

/// The value of DOM attributes.
#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    String(CowStr),
    Bool(bool),
}

macro_rules! impl_attributes {
    ($(
        $Variant:ident => [ $($t:ty),* $(,)? ];
    )*) => {$(
        $(
            impl From<$t> for Attribute {
                fn from(val: $t) -> Self {
                    Attribute::$Variant(val.into())
                }
            }
        )*
    )*};
}

impl_attributes! {
    String => [
        &'static str,
        String,
        CowStr,
    ];
    Bool => [bool];
}

/// The property values in DOM object.
#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    String(CowStr),
    Number(f64),
    Bool(bool),
}

macro_rules! impl_properties {
    ($(
        $Variant:ident => [ $($t:ty),* $(,)? ];
    )*) => {$(
        $(
            impl From<$t> for Property {
                fn from(val: $t) -> Self {
                    Property::$Variant(val.into())
                }
            }
        )*
    )*};
}

impl_properties! {
    String => [
        &'static str,
        String,
        CowStr,
    ];
    Number => [
        f64, f32,
        i8, i16, i32,
        u8, u16, u32,
    ];
    Bool => [bool];
}
