#![feature(generic_associated_types)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(box_syntax)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]

trait Class {}

trait ClassMember<C: Class> {}

trait Morphism {
    type Domain;
    type Codomain;
    fn call(self, arg: Self::Domain) -> Self::Codomain;
}

trait Endomorphism: Morphism<Codomain = <Self as Morphism>::Domain> {
    type Identity: Endomorphism;
    const IDENTITY: Self::Identity;
}

trait HomClass: Class {
    type Domains;
}

trait HomClassMember<Homs: HomClass>: Morphism {}

struct RustStructs {}
impl Class for RustStructs {}
impl<A> ClassMember<RustStructs> for A {}

// impl<F, Args, Outs> Test for F
// where
// F: Fn<Args> -> Outs
// のように書けない。これは、https://github.com/rust-lang/rust/issues/25041で管理されてる
// そのため、Function<F, I, O>でwrapして、型引数を使うようにする

pub struct Function<F, I, O>
where
    F: FnOnce(I) -> O,
{
    f: F,
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

struct RustFunctions {}

impl Class for RustFunctions {}
impl HomClass for RustFunctions {
    type Domains = RustStructs;
}

impl<F, I, O> ClassMember<RustFunctions> for Function<F, I, O> where F: FnOnce(I) -> O {}

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

fn id<X>(x: X) -> X {
    x
}

impl<F, I> Endomorphism for Function<F, I, I>
where
    F: FnOnce(I) -> I,
{
    type Identity = Function<fn(I) -> I, I, I>;
    const IDENTITY: Self::Identity = Function::new(id);
}

impl<F, I, O> HomClassMember<RustFunctions> for Function<F, I, O> where F: FnOnce(I) -> O {}

trait Composition<Lhs: Morphism, Rhs: Morphism> {
    type Output: Morphism;
    fn compose(lhs: Lhs, rhs: Rhs) -> Self::Output;
}

struct ArrayComposition {}

// impl<F1, F2, A, B, C> Composition<Function<F1, B, C>, Function<F2, A, B>> for ArrayComposition
// where
//     F1: FnOnce(B) -> C,
//     F2: FnOnce(A) -> B,
// {
//     type Output = Function<impl FnOnce(A) -> C, A, C>;
//     fn compose(lhs: Function<F1, B, C>, rhs: Function<F2, A, B>) -> Self::Output {
//         let f = move |arg| lhs.call(rhs.call(arg));
//         Function::new(f)
//     }
// }

impl<L, R> Composition<L, R> for ArrayComposition
where
    L: HomClassMember<RustFunctions> + Morphism<Domain = <R as Morphism>::Codomain>,
    R: HomClassMember<RustFunctions>,
{
    type Output = impl HomClassMember<RustFunctions>
        + Morphism<Domain = <R as Morphism>::Domain, Codomain = <L as Morphism>::Codomain>;
    fn compose(lhs: L, rhs: R) -> Self::Output {
        let f = move |arg| lhs.call(rhs.call(arg));
        Function::new(f)
    }
}

trait Category {
    // class of objects
    type Objects: Class;

    // morphism class between objects
    type Morphisms: HomClass<Domains = Self::Objects>;

    // composition of morphism
    type Composer<L: HomClassMember<Self::Morphisms>, R: HomClassMember<Self::Morphisms> + Morphism<Codomain = <L as Morphism>::Domain>>: Composition<L, R>;

    // category must have identity morphism
    type Identity<Item>: HomClassMember<Self::Morphisms> + Morphism<Domain = Item, Codomain = Item>;

    fn identity<Item>() -> Self::Identity<Item>;
}

pub struct RustCategory {}

impl const Category for RustCategory {
    type Objects = RustStructs;
    type Morphisms = RustFunctions;
    type Composer<
        L: HomClassMember<Self::Morphisms>,
        R: HomClassMember<Self::Morphisms> + Morphism<Codomain = <L as Morphism>::Domain>,
    > = ArrayComposition;

    type Identity<Item> = Function<fn(Item) -> Item, Item, Item>;

    fn identity<Item>() -> Self::Identity<Item> {
        <Self as Category>::Identity::<Item>::IDENTITY
    }
}

trait Ob<Cat: Category> = ClassMember<<Cat as Category>::Objects>;

trait Functor {
    type Source: Category;
    type Target: Category;
    type Map<A>;

    fn map<A>(a: A) -> Self::Map<A>
    where
        A: Ob<Self::Source>,
        Self::Map<A>: Ob<Self::Target>;
}

struct OptionFunctor {}

impl Functor for OptionFunctor {
    type Source = RustCategory;
    type Target = RustCategory;
    type Map<A> = Option<A>;
    fn map<A>(a: A) -> Option<A>
    where
        A: Ob<Self::Source>,
    {
        Some(a)
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
