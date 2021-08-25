pub trait Class {}

pub trait ClassMember<C: Class> {}

pub trait Morphism {
    type Domain;
    type Codomain;
    fn call(self, arg: Self::Domain) -> Self::Codomain;
}

pub type Domain<F> = <F as Morphism>::Domain;
pub type Codomain<F> = <F as Morphism>::Codomain;

pub trait Endomorphism: Morphism<Codomain = <Self as Morphism>::Domain> {
    type Identity: Endomorphism;
    const IDENTITY: Self::Identity;
}

pub trait HomClass: Class {
    type Domains: Class;
}

pub trait HomClassMember<Homs: HomClass>: Morphism
where
    Domain<Self>: ClassMember<<Homs as HomClass>::Domains>,
    Codomain<Self>: ClassMember<<Homs as HomClass>::Domains>
{
}

pub trait Composition<Lhs: Morphism, Rhs: Morphism> {
    type Output: Morphism;
    fn compose(lhs: Lhs, rhs: Rhs) -> Self::Output;
}

pub trait Category {
    // class of objects
    type Objects: Class;

    // morphism class between objects
    type Morphisms: HomClass<Domains = Self::Objects>;

    // composition of morphism
    type Composer<L: HomClassMember<Self::Morphisms>, R: HomClassMember<Self::Morphisms> + Morphism<Codomain = <L as Morphism>::Domain>>: Composition<L, R>;

    // category must have identity morphism
    type Identity<Item: ClassMember<Self::Objects>>: HomClassMember<Self::Morphisms> + Endomorphism + Morphism<Domain = Item>;

    fn identity<Item>() -> Self::Identity<Item>
    where
        Domain<Self::Identity<Item>>: ClassMember<Self::Objects>;
}

pub trait Ob<Cat: Category>: ClassMember<<Cat as Category>::Objects> {}
impl<A, Cat> Ob<Cat> for A
where
    A: ClassMember<<Cat as Category>::Objects>,
    Cat: Category,
{
}

pub trait Hom<Cat: Category>: 
where
    Self: HomClassMember<<Cat as Category>::Morphisms>,
    Domain<Self>: ClassMember<<Cat as Category>::Objects>,
    Codomain<Self>: ClassMember<<Cat as Category>::Objects>,
{}
impl<A, Cat> Hom<Cat> for A
where
    A: HomClassMember<<Cat as Category>::Morphisms>,
    Domain<A>: ClassMember<<Cat as Category>::Objects>,
    Codomain<A>: ClassMember<<Cat as Category>::Objects>,
    Cat: Category,
{
}

pub trait CovariantFunctor {
    type Source: Category;
    type Target: Category;
    type Map<A>: Ob<Self::Target>
    where
        A: Ob<Self::Source>;

    type FMap<F>: Hom<Self::Target>
    where
        F: Hom<Self::Source>,
        Domain<Self::FMap<F>>: ClassMember<<Self::Target as Category>::Objects>,
        Codomain<Self::FMap<F>>: ClassMember<<Self::Target as Category>::Objects>,
        ;

    fn map<A>(a: A) -> Self::Map<A>
    where
        A: Ob<Self::Source>,
        // Self::Map<A>: Ob<Self::Target>
        ;

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: Hom<Self::Source>,
        Domain<F>: ClassMember<<Self::Source as Category>::Objects>,
        Codomain<F>: ClassMember<<Self::Source as Category>::Objects>,
        Domain<Self::FMap<F>>: ClassMember<<Self::Target as Category>::Objects>,
        Codomain<Self::FMap<F>>: ClassMember<<Self::Target as Category>::Objects>,
        // Self::FMap<F>: Hom<Self::Target>
        // cannot uncommented this line because rustc can't resolve F trait bounds in implement
    ;
}

pub trait EndoFunctor: CovariantFunctor<Source = <Self as CovariantFunctor>::Target> {}
