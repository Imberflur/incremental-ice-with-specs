use core::any::Any;
use core::marker::PhantomData;

struct DerefWrap<T>(T);

impl<T> core::ops::Deref for DerefWrap<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Storage<T, D> {
    phantom: PhantomData<(T, D)>,
}

type ReadStorage<T> = Storage<T, DerefWrap<MaskedStorage<T>>>;

pub trait Component {
    type Storage;
}

struct VecStorage;

struct Pos;

impl Component for Pos {
    type Storage = VecStorage;
}

struct GenericComp<T> {
    _t: T,
}

impl<T: 'static> Component for GenericComp<T> {
    type Storage = VecStorage;
}
struct ReadData {
    pos_interpdata: ReadStorage<GenericComp<Pos>>,
}

trait System {
    type SystemData;

    fn run(data: Self::SystemData, any: Box<dyn Any>);
}

struct Sys;

impl System for Sys {
    type SystemData = (ReadData, ReadStorage<Pos>);

    fn run((data, pos): Self::SystemData, any: Box<dyn Any>) {
        <ReadStorage<GenericComp<Pos>> as SystemData>::setup(any);

        ParJoin::par_join((&pos, &data.pos_interpdata));
    }
}

trait ParJoin {
    fn par_join(self)
    where
        Self: Sized,
    {
    }
}

impl<'a, T, D> ParJoin for &'a Storage<T, D>
where
    T: Component,
    D: core::ops::Deref<Target = MaskedStorage<T>>,
    T::Storage: Sync,
{
}

impl<A, B> ParJoin for (A, B)
where
    A: ParJoin,
    B: ParJoin,
{
}

pub trait SystemData {
    fn setup(any: Box<dyn Any>);
}

impl<T: 'static> SystemData for ReadStorage<T>
where
    T: Component,
{
    fn setup(mut any: Box<dyn Any>) {
        let fetch: &mut (MetaTable<dyn Any>, MaskedStorage<T>) = any.downcast_mut().unwrap();

        fetch.0.register(&fetch.1);
    }
}

pub struct MaskedStorage<T: Component> {
    _inner: T::Storage,
}

struct Invariant<T: ?Sized>(*mut T);

unsafe impl<T> Send for Invariant<T> where T: ?Sized {}

unsafe impl<T> Sync for Invariant<T> where T: ?Sized {}

pub unsafe trait CastFrom<T> {
    fn cast(t: &T) -> &Self;
}

pub struct MetaTable<T: ?Sized> {
    marker: PhantomData<Invariant<T>>,
}

impl<T: ?Sized> MetaTable<T> {
    pub fn register<R>(&mut self, r: &R)
    where
        T: CastFrom<R> + 'static,
    {
        <T as CastFrom<R>>::cast(r);
    }
}

unsafe impl<T> CastFrom<T> for dyn Any
where
    T: Any + 'static,
{
    fn cast(t: &T) -> &Self {
        t
    }
}
