use crate::{another_trait::AnotherTrait, GenericThing};

#[entrait::entrait(pub OneTrait, mock_api=OneTraitMock)]
pub fn one_fn<T: GenericThing, D>(_: &D, _: T)
where
    D: AnotherTrait<T>,
{
    println!("OneTrait");
}

#[cfg(test)]
mod tests {

    use crate::{another_trait::AnotherTraitMock, ConcreteThing};
    use unimock::{matching, MockFn, Unimock};

    #[test]
    fn test_entrait_private_type_problem() {
        let mocked_deps = Unimock::new(AnotherTraitMock
            .with_types::<ConcreteThing>()
            .some_call(matching!(_))
            .answers(&|_, _| ()),);
    }
}
