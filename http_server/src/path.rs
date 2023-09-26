use std::fmt::Debug;

pub trait Path : Sized + Debug where Self::Error: Debug {
    type Error;

    fn parse<S: AsRef<str>>(path: S) -> Result<Self, Self::Error>;
}

impl Path for String {
    type Error = ();

    fn parse<S: AsRef<str>>(path: S) -> Result<Self, Self::Error> {
        Ok(path.as_ref().to_string())
    }
}


