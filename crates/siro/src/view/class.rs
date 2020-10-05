use super::{ModifyView, View};
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VNode},
};

pub fn class(class_name: impl Into<CowStr>) -> Class {
    Class {
        class_name: class_name.into(),
    }
}

pub struct Class {
    class_name: CowStr,
}

impl<TView> ModifyView<TView> for Class
where
    TView: View,
{
    type Msg = TView::Msg;
    type View = WithClass<TView>;

    fn modify(self, view: TView) -> Self::View {
        WithClass {
            view,
            class_name: self.class_name,
        }
    }
}

pub struct WithClass<T> {
    view: T,
    class_name: CowStr,
}

impl<TView> View for WithClass<TView>
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
                element.classes.insert(self.class_name);
                element.into()
            }
            node => node,
        }
    }
}
