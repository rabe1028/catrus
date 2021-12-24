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

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

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
        Self::FMap<F>: Hom<RustCategory>,
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
    fn test_map() {
        let a = OptionFunctor::map(1u32);
        let mapfn = OptionFunctor::fmap(Function::new(|a| a + 4));
        assert_eq!(Some(5), mapfn.call(a));
    }

    #[quickcheck]
    fn functor_raw_identity(x: u32) -> bool {
        let id = <<OptionFunctor as CovariantFunctor>::Source as Category>::identity::<_>();
        let mapid = OptionFunctor::fmap(id.clone());
        OptionFunctor::map(id.call(x)) == mapid.call(OptionFunctor::map(x))
    }
}
