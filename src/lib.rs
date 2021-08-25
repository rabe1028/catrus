#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]
#![feature(associated_type_defaults)]
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

    // we need to add trait bound "HomClassMember<RustFunctions>"
    // because rustc maybe cannot inferred trait bound
    type FMap<F: Hom<RustCategory> + HomClassMember<RustFunctions>> = Function<
        impl FnOnce(Option<Dom<F, RustFunctions>>) -> Option<Cod<F, RustFunctions>>,
        Option<Dom<F, RustFunctions>>,
        Option<Cod<F, RustFunctions>>,
    >;

    fn map<A>(a: A) -> Option<A>
    where
        A: Ob<Self::Source>,
    {
        Some(a)
    }

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: Hom<RustCategory> 
        + HomClassMember<RustFunctions> 
        + Morphism<
            Domain = <F as HomClassMember<RustFunctions>>::Domain, 
            Codomain=<F as HomClassMember<RustFunctions>>::Codomain
            >,
        Self::FMap<F>: Hom<RustCategory> 
        + HomClassMember<RustFunctions, 
            Domain = Option<Dom<F, RustFunctions>>, 
            Codomain = Option<Cod<F, RustFunctions>>
            >,
    {
        let f = move |arg: Option<Dom<F, RustFunctions>>| -> Option<Cod<F, RustFunctions>> {
            arg.map(|a| Morphism::call(f, a))
        };
        Function::new(f)
    }
}
