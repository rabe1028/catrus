use crate::axiom::*;

pub struct Function<F, I, O>
where
    F: FnOnce(I) -> O,
{
    pub f: F,
    in_phantom: std::marker::PhantomData<I>,
    out_phantom: std::marker::PhantomData<O>,
}

impl<F, I, O> Function<F, I, O>
where
    F: FnOnce(I) -> O,
{
    pub const fn new(f: F) -> Self {
        Function {
            f,
            in_phantom: std::marker::PhantomData,
            out_phantom: std::marker::PhantomData,
        }
    }
}

impl<F, I, O> Morphism for Function<F, I, O>
where
    F: FnOnce(I) -> O,
{
    type Domain = I;
    type Codomain = O;
    fn call(self, arg: I) -> O {
        let function = self.f;
        function(arg)
    }
}

impl<F, I> Endomorphism for Function<F, I, I>
where
    F: FnOnce(I) -> I,
{
    type Identity = Function<fn(I) -> I, I, I>;
    const IDENTITY: Self::Identity = Function::new(id);
}

pub fn id<X>(x: X) -> X {
    x
}
