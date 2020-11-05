use std::borrow::Cow;
use wasm_bindgen::JsValue;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    CaughtFromJS(JsValue),
    Custom(Cow<'static, str>),
}

impl Error {
    pub fn caught_from_js(payload: JsValue) -> Self {
        Self {
            kind: ErrorKind::CaughtFromJS(payload),
        }
    }

    pub fn custom(msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind: ErrorKind::Custom(msg.into()),
        }
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        match error.kind {
            ErrorKind::CaughtFromJS(payload) => payload,
            ErrorKind::Custom(msg) => JsValue::from_str(&*msg),
        }
    }
}
