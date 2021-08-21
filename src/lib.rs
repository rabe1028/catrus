#![feature(generic_associated_types)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(box_syntax)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]

pub mod axiom;
use axiom::*;

pub mod wrapper;
use wrapper::*;

pub mod rust_category;
use rust_category::*;

struct OptionFunctor {}

impl Functor for OptionFunctor {
    type Source = RustCategory;
    type Target = RustCategory;

    type Map<A> = Option<A>;
    type FMap<F: Hom<RustCategory>> = Function<
        impl FnOnce(Option<Domain<F>>) -> Option<Codomain<F>>,
        Option<Domain<F>>,
        Option<Codomain<F>>,
    >;

    fn map<A>(a: A) -> Option<A>
    where
        A: Ob<Self::Source>,
    {
        Some(a)
    }

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: Hom<RustCategory>,
    {
        let f = move |arg: Option<Domain<F>>| -> Option<Codomain<F>> {
            arg.map(|a| Morphism::call(f, a))
        };
        Function::new(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_closure_compose() {
        let a = Function::new(|x: usize| x + 1);
        let b = Function::new(|x: usize| x * x);
        // let c = <<RustCategory as Category>::Composer<_, _> as Composition<_, _>>::compose(a, b);
        let _ = ArrayComposition::compose(a, b);
    }
}
