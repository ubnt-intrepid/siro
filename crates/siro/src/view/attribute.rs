use super::{ModifyView, View};
use crate::{
    mailbox::Mailbox,
    vdom::{self, CowStr, VNode},
};

pub fn attribute(name: impl Into<CowStr>, value: impl Into<vdom::Attribute>) -> Attribute {
    Attribute {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Attribute {
    name: CowStr,
    value: vdom::Attribute,
}

impl<TView> ModifyView<TView> for Attribute
where
    TView: View,
{
    type Msg = TView::Msg;
    type View = WithAttribute<TView>;

    fn modify(self, view: TView) -> Self::View {
        WithAttribute {
            view,
            name: self.name,
            value: self.value,
        }
    }
}

pub struct WithAttribute<T> {
    view: T,
    name: CowStr,
    value: vdom::Attribute,
}

impl<TView> View for WithAttribute<TView>
where
    TView: View,
{
    type Msg = TView::Msg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        match self.view.render(mailbox) {
            VNode::Element(mut element) => {
                element.attributes.insert(self.name, self.value);
                element.into()
            }
            node => node,
        }
    }
}
