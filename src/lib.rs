use specs::{
    shred::ResourceId, Component, Entities, ParJoin, ReadStorage, System, SystemData, World,
};

pub struct Pos;

impl Component for Pos {
    type Storage = specs::VecStorage<Self>;
}

pub struct InterpBuffer<T> {
    _t: T,
}

impl<T: 'static + Send + Sync> Component for InterpBuffer<T> {
    type Storage = specs::VecStorage<Self>;
}

#[derive(SystemData)]
pub struct ReadData<'a> {
    pos_interpdata: ReadStorage<'a, InterpBuffer<Pos>>,
}

#[derive(Default)]
pub struct Sys;

impl<'a> System<'a> for Sys {
    type SystemData = (ReadData<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (data, pos): Self::SystemData) {
        (&pos, &data.pos_interpdata).par_join();
    }
}
