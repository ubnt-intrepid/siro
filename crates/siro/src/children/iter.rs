use super::{Children, Context};
use crate::vdom::Node;
use wasm_bindgen::JsValue;

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
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        for child in self.iter {
            ctx.append_child(child)?;
        }
        Ok(())
    }
}
