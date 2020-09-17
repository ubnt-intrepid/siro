use super::{cache::Key, element::Element, text::Text};

pub enum Node {
    Element(Element),
    Text(Text),
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

macro_rules! impl_from_strs {
    ($( $t:ty ),*) => {$(
        impl From<$t> for Node {
            fn from(value: $t) -> Self {
                Self::Text(Text::from(value))
            }
        }
    )*};
}

impl_from_strs!(&'static str, String, std::borrow::Cow<'static, str>);

impl Node {
    pub(super) fn key(&self) -> Key {
        match self {
            Node::Element(e) => e.key(),
            Node::Text(t) => t.key(),
        }
    }
}
