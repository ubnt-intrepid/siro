use super::{ModifyView, View};
use crate::{
    mailbox::Mailbox,
    vdom::{self, CowStr, VNode},
};

pub fn property(name: impl Into<CowStr>, value: impl Into<vdom::Property>) -> Property {
    Property {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Property {
    name: CowStr,
    value: vdom::Property,
}

impl<TView> ModifyView<TView> for Property
where
    TView: View,
{
    type Msg = TView::Msg;
    type View = WithProperty<TView>;

    fn modify(self, view: TView) -> Self::View {
        WithProperty {
            view,
            name: self.name,
            value: self.value,
        }
    }
}

pub struct WithProperty<T> {
    view: T,
    name: CowStr,
    value: vdom::Property,
}

impl<TView> View for WithProperty<TView>
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
                element.properties.insert(self.name, self.value);
                element.into()
            }
            node => node,
        }
    }
}
