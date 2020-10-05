use super::{ModifyView, View};
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VNode},
};

pub fn style(name: impl Into<CowStr>, value: impl Into<CowStr>) -> Style {
    Style {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Style {
    name: CowStr,
    value: CowStr,
}

impl<TView> ModifyView<TView> for Style
where
    TView: View,
{
    type Msg = TView::Msg;
    type View = WithStyle<TView>;

    fn modify(self, view: TView) -> Self::View {
        WithStyle {
            view,
            name: self.name,
            value: self.value,
        }
    }
}

pub struct WithStyle<T> {
    view: T,
    name: CowStr,
    value: CowStr,
}

impl<TView> View for WithStyle<TView>
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
                element.styles.insert(self.name, self.value);
                element.into()
            }
            node => node,
        }
    }
}
