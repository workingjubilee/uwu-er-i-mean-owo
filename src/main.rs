struct SomeType {}
impl SomeType {
    fn less_trivial_constructor(_a: ()) -> Self {
        SomeType {}
    }
}
const SOME_CONSTANT: () = ();

#[macro_use]
macro_rules! uwuify {
    ($($this:ident),*) => {
        $(
        impl WhatsThis for $this {
            fn uwu() -> Self {
                <$this as Default>::default()
            }
        }
        )*
    }
}
mod ty0;
mod ty1;
mod ty2;
mod ty3;
mod ty4;
mod ty5;
mod ty6;
mod ty7;
mod ty8;
mod ty9;

trait WhatsThis {
    fn owo() -> Self;
}

impl WhatsThis for SomeType {
    fn owo() -> Self {
        SomeType::less_trivial_constructor(SOME_CONSTANT)
    }
}

fn main() {
    println!("Hello, world!");
}
