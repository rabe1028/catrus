use crate::axiom::*;
use crate::wrapper::*;

pub struct RustStructs {}

impl Class for RustStructs {}
impl<A> ClassMember<RustStructs> for A {}

pub struct RustFunctions {}

impl Class for RustFunctions {}
impl HomClass for RustFunctions {
    type Domains = RustStructs;
}

impl<F, I, O> ClassMember<RustFunctions> for Function<F, I, O> where F: FnOnce(I) -> O {}
impl<F, I, O> HomClassMember<RustFunctions> for Function<F, I, O> where
    F: FnOnce(I) -> O,
    I: ClassMember<RustStructs>,
    O: ClassMember<RustStructs>
{
}

pub struct ArrayComposition {}

impl<L, R> Composition<L, R> for ArrayComposition
where
    L: HomClassMember<RustFunctions> + Morphism<Domain = <R as Morphism>::Codomain>,
    R: HomClassMember<RustFunctions>,
{
    type Output =
        impl HomClassMember<RustFunctions> + Morphism<Domain = Domain<R>, Codomain = Codomain<L>>;
    fn compose(lhs: L, rhs: R) -> Self::Output {
        let f = move |arg| lhs.call(rhs.call(arg));
        Function::new(f)
    }
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

    #[test]
    fn test_identity() {
        let a = 1;
        let func = RustCategory::identity();
        assert_eq!(a, func.call(a))
    }

    #[test]
    fn test_const_identity() {
        const FUNC: Function<fn(i32) -> i32, i32, i32> = RustCategory::identity();
        const A: i32 = 1;
        assert_eq!(A, FUNC.call(A))
    }
}
