#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]
// #![recursion_limit = "1024"]

pub mod axiom;
use axiom::*;

pub mod wrapper;
use wrapper::*;

pub mod rust_category;
use rust_category::*;

struct OptionFunctor {}

impl CovariantFunctor for OptionFunctor {
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
        // Self::FMap<F>: Hom<RustCategory>,
    {
        let f = move |arg: Option<Domain<F>>| -> Option<Codomain<F>> {
            arg.map(|a| Morphism::call(f, a))
        };
        Function::new(f)
    }
}
