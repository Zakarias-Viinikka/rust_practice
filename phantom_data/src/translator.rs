/*use std::marker::PhantomData;

pub struct ColMarker<'a, T> {
    column_name: &'a str,
    marker: PhantomData<T>,
}

pub trait DoSomething {
    type ReturnValue;
    fn destruct_col() -> SpecificEnum;
}

*/

pub enum SpecificEnum {
    Variant1(String),
    Variant2(i32),
}

fn get_generic() -> SpecificEnum {
    SpecificEnum::Variant2(3)
}

fn get_specific<T>() -> Result<T, String> {
    get_generic().parse_as::<T>()
}

fn parse_as<T: AllowedT>(thing_to_parse: SpecificEnum) -> Result<T, String> {
    match thing_to_parse {
        SpecificEnum::Variant1(s) => Ok(s.try_into()?),
        SpecificEnum::Variant2(i) => Ok(i.try_into()?),
    }
}

trait AllowedT {}

/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
