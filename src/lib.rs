use specs::shred::Fetch;
use specs::shred::MetaTable;
use specs::storage::AnyStorage;
use specs::storage::MaskedStorage;
use specs::Component;
use specs::Storage;
use specs::World;

pub type ReadStorage<'a, T> = Storage<'a, T, Fetch<'a, MaskedStorage<T>>>;

trait System<'a> {
    type SystemData;

    fn run(data: Self::SystemData, world: &mut World);
}

struct Pos;

impl Component for Pos {
    type Storage = specs::VecStorage<Self>;
}

struct InterpBuffer<T> {
    _t: T,
}

impl<T: 'static + Send + Sync> Component for InterpBuffer<T> {
    type Storage = specs::VecStorage<Self>;
}
struct ReadData<'a> {
    pos_interpdata: ReadStorage<'a, InterpBuffer<Pos>>,
}

struct Sys;

impl<'a> System<'a> for Sys {
    type SystemData = (ReadData<'a>, ReadStorage<'a, Pos>);

    fn run((data, pos): Self::SystemData, world: &mut World) {
        <ReadStorage<'a, InterpBuffer<Pos>> as SystemData>::setup(world);

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

impl<'a, 'e, T, D> ParJoin for &'a specs::Storage<'e, T, D>
where
    T: specs::Component,
    D: core::ops::Deref<Target = specs::storage::MaskedStorage<T>>,
    T::Storage: Sync,
{
}

impl<A, B> ParJoin for (A, B)
where
    A: ParJoin,
    B: ParJoin,
{
}

//
//pub trait Component: Any + Sized {
//    type Storage: UnprotectedStorage<Self> + Any + Send + Sync;
//}

pub trait SystemData<'a> {
    /// Sets up the system data for fetching it from the `World`.
    fn setup(world: &mut World);
}

impl<'a, T> SystemData<'a> for ReadStorage<'a, T>
where
    T: Component,
{
    fn setup(res: &mut World) {
        res.fetch_mut::<MetaTable<dyn AnyStorage>>()
            .register(&*res.fetch::<MaskedStorage<T>>());
    }
}
/*
pub struct MaskedStorage<T: Component> {
    mask: specs::hibitset::BitSet,
    inner: T::Storage,
}*/
