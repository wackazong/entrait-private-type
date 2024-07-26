mod another_trait;
mod one_trait;

pub trait GenericThing {}

struct ConcreteThing;

impl GenericThing for ConcreteThing {}

fn main() {}
