use crate::vdom::{Children, ElementContext, Node};

/// Create a `Children` from an iterator.
pub fn iter<I>(iter: I) -> Iter<I::IntoIter>
where
    I: IntoIterator,
    I::Item: Node,
{
    Iter {
        iter: iter.into_iter(),
    }
}

pub struct Iter<I> {
    iter: I,
}

impl<TMsg: 'static, I> Children<TMsg> for Iter<I>
where
    I: Iterator,
    I::Item: Node<Msg = TMsg>,
{
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        for child in self.iter {
            ctx.child(child)?;
        }
        Ok(())
    }
}
