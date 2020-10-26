use super::{Nodes, NodesRenderer};
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
            Nodes::render_nodes(child, &mut renderer)?;
        }
        renderer.end()
    }
}
