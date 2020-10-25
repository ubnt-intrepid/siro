use super::{Node, Nodes, NodesRenderer};
use std::marker::PhantomData;

/// Create a `Nodes` from an iterator.
pub fn iter<I, TMsg>(iter: I) -> impl Nodes<TMsg>
where
    I: IntoIterator,
    I::Item: Nodes<TMsg>,
    TMsg: 'static,
{
    Iter {
        iter: iter.into_iter(),
        _marker: PhantomData,
    }
}

struct Iter<I, TMsg> {
    iter: I,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<I, TMsg> Nodes<TMsg> for Iter<I, TMsg>
where
    I: Iterator,
    I::Item: Nodes<TMsg>,
    TMsg: 'static,
{
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        for child in self.iter {
            Nodes::render_nodes(child, IterContext { ctx: &mut renderer })?;
        }
        renderer.end()
    }
}

struct IterContext<'a, Ctx: ?Sized> {
    ctx: &'a mut Ctx,
}

impl<Ctx: ?Sized> NodesRenderer for IterContext<'_, Ctx>
where
    Ctx: NodesRenderer,
{
    type Msg = Ctx::Msg;
    type Ok = ();
    type Error = Ctx::Error;

    #[inline]
    fn child<N>(&mut self, child: N) -> Result<(), Self::Error>
    where
        N: Node<Msg = Self::Msg>,
    {
        self.ctx.child(child)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
