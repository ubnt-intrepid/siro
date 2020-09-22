use std::rc::Rc;

pub struct Callback<In>(Rc<dyn Fn(In)>);

impl<F, In> From<F> for Callback<In>
where
    F: Fn(In) + 'static,
{
    fn from(f: F) -> Self {
        Callback(Rc::new(f))
    }
}

impl<In> Callback<In> {
    pub(crate) fn invoke(&self, arg: In) {
        (self.0)(arg);
    }
}
