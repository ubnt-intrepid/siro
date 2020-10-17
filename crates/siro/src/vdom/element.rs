use super::{text::text, Context, CowStr, ElementContext, Node};
use either::Either;
use std::marker::PhantomData;

/// Create a virtual node corresponding to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element).
pub fn element<TMsg: 'static>(
    tag_name: impl Into<CowStr>,
    namespace_uri: Option<CowStr>,
    attr: impl Attr<TMsg>,
    children: impl Children<TMsg>,
) -> impl Node<Msg = TMsg> {
    Element {
        tag_name: tag_name.into(),
        namespace_uri,
        attr,
        children,
        _marker: PhantomData,
    }
}

struct Element<TMsg, A, C> {
    tag_name: CowStr,
    namespace_uri: Option<CowStr>,
    attr: A,
    children: C,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static, A, C> Node for Element<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Children<TMsg>,
{
    type Msg = TMsg;

    fn render<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        let mut element = ctx.element_node(self.tag_name, self.namespace_uri)?;
        self.attr.apply(&mut element)?;
        self.children.diff(&mut element)?;
        element.end()
    }
}

// ==== Attr ====

/// The modifier of a `View` that annotates one or more attribute values.
pub trait Attr<TMsg: 'static> {
    /// Apply itself to specified `VElement`.
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>;
}

impl<TMsg: 'static> Attr<TMsg> for () {
    fn apply<Ctx: ?Sized>(self, _: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        Ok(())
    }
}

macro_rules! impl_attr_for_tuples {
    ( $H:ident, $( $T:ident ),* ) => {
        impl<TMsg: 'static, $H, $( $T ),*> Attr<TMsg> for ($H, $( $T ),*)
        where
            $H: Attr<TMsg>,
            $( $T: Attr<TMsg>, )*
        {
            fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
            where
                Ctx: ElementContext<Msg = TMsg>,
            {
                #[allow(non_snake_case)]
                let ($H, $( $T ),*) = self;
                $H.apply(ctx)?;
                $( $T.apply(ctx)?; )*
                Ok(())
            }
        }

        impl_attr_for_tuples!($($T),*);
    };

    ( $T:ident ) => {
        impl<TMsg: 'static, $T> Attr<TMsg> for ($T,)
        where
            $T: Attr<TMsg>,
        {
            fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
            where
                Ctx: ElementContext<Msg = TMsg>,
            {
                self.0.apply(ctx)?;
                Ok(())
            }
        }
    };
}

impl_attr_for_tuples!(
    M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, //
    M11, M12, M13, M14, M15, M16, M17, M18, M19, M20
);

impl<TMsg: 'static, T> Attr<TMsg> for Option<T>
where
    T: Attr<TMsg>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        match self {
            Some(m) => m.apply(ctx),
            None => Ok(()),
        }
    }
}

impl<TMsg: 'static, M1, M2> Attr<TMsg> for Either<M1, M2>
where
    M1: Attr<TMsg>,
    M2: Attr<TMsg>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Attr::apply(l, ctx),
            Either::Right(r) => Attr::apply(r, ctx),
        }
    }
}

// ==== Children ====

/// A trait that represents a set of child nodes.
pub trait Children<TMsg: 'static> {
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>;
}

impl<TMsg: 'static> Children<TMsg> for () {
    fn diff<Ctx: ?Sized>(self, _: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        Ok(())
    }
}

impl<TMsg: 'static> Children<TMsg> for &'static str {
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        Children::diff(text(self), ctx)
    }
}

impl<TMsg: 'static> Children<TMsg> for String {
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        Children::diff(text(self), ctx)
    }
}

impl<TMsg: 'static, C> Children<TMsg> for C
where
    C: Node<Msg = TMsg>,
{
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.child(self)?;
        Ok(())
    }
}

macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl<TMsg: 'static, $H, $($T),+ > Children<TMsg> for ( $H, $($T),+ )
        where
            $H: Children<TMsg>,
            $( $T: Children<TMsg>, )+
        {
            #[allow(non_snake_case)]
            fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
            where
                Ctx: ElementContext<Msg = TMsg>,
            {
                let ($H, $($T),+) = self;
                Children::diff($H, ctx)?;
                $( Children::diff($T, ctx)?; )+
                Ok(())
            }
        }

        impl_children_for_tuples!( $($T),+ );
    };

    ( $C:ident ) => {
        impl<TMsg: 'static, $C > Children<TMsg> for ( $C, )
        where
            $C: Children<TMsg>,
        {
            #[allow(non_snake_case)]
            fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
            where
                Ctx: ElementContext<Msg = TMsg>,
            {
                let ($C,) = self;
                Children::diff($C, ctx)?;
                Ok(())
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);

impl<TMsg: 'static, T> Children<TMsg> for Option<T>
where
    T: Children<TMsg>,
{
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        match self {
            Some(ch) => Children::diff(ch, ctx),
            None => Ok(()),
        }
    }
}

impl<TMsg: 'static, M1, M2> Children<TMsg> for Either<M1, M2>
where
    M1: Children<TMsg>,
    M2: Children<TMsg>,
{
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Children::diff(l, ctx),
            Either::Right(r) => Children::diff(r, ctx),
        }
    }
}
