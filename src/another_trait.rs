use crate::{one_trait::OneTrait, GenericThing};

#[entrait::entrait(pub AnotherTrait, mock_api=AnotherTraitMock)]
pub fn another_fn<T: GenericThing, D>(_: &D, _: T)
where
    D: OneTrait<T>,
{
    println!("AnotherTrait");
}

