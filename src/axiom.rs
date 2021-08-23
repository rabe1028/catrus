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
    Self: Morphism<Domain = <Self as HomClassMember<Homs>>::Domain, Codomain=<Self as HomClassMember<Homs>>::Codomain>,
    Domain<Self>: ClassMember<<Homs as HomClass>::Domains>,
    Codomain<Self>: ClassMember<<Homs as HomClass>::Domains>
{
    type Domain: ClassMember<<Homs as HomClass>::Domains>;
    type Codomain: ClassMember<<Homs as HomClass>::Domains>;
}

pub type Dom<F, Homs> = <F as HomClassMember<Homs>>::Domain;
pub type Cod<F, Homs> = <F as HomClassMember<Homs>>::Codomain;

pub trait EndomorphHomClassMember<Homs: HomClass>: Endomorphism
where
    Self: HomClassMember<Homs, Codomain = <Self as HomClassMember<Homs>>::Domain>
{}

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
    type Composer<L: HomClassMember<Self::Morphisms>, R: HomClassMember<Self::Morphisms, Codomain = <L as HomClassMember<Self::Morphisms>>::Domain>>: Composition<L, R>;

    // category must have identity morphism
    type Identity<Item>: EndomorphHomClassMember<Self::Morphisms>;

    fn identity<Item>() -> Self::Identity<Item>
    where
        Domain<Self::Identity<Item>>: ClassMember<Self::Objects>;
}

pub trait Ob<Cat: Category> : ClassMember<<Cat as Category>::Objects> {}
impl <A, Cat> Ob<Cat> for A
where
    A: ClassMember<<Cat as Category>::Objects>,
    Cat: Category,
    {}

pub trait Hom<Cat: Category>: HomClassMember<<Cat as Category>::Morphisms> {}
impl <A, Cat> Hom<Cat> for A
where
    A: HomClassMember<<Cat as Category>::Morphisms>,
    Cat: Category,
    {}

pub trait CovariantFunctor {
    type Source: Category;
    type Target: Category;
    type Map<A>: Ob<Self::Target>
    where
        A: Ob<Self::Source>;

    type FMap<F>
    : // Hom<Self::Target>
    where
        F: Hom<Self::Source>;

    fn map<A>(a: A) -> Self::Map<A>
    where
        A: Ob<Self::Source>,
        Self::Map<A>: Ob<Self::Target>;

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: Hom<Self::Source>,
        Self::FMap<F>: Hom<Self::Target>
    ;
}

pub trait EndoFunctor: CovariantFunctor<Source = <Self as CovariantFunctor>::Target> {}
