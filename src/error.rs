/// Render error return type.
#[derive(Debug)]
pub enum Error {
    /// Count of categories on scale doesn't equal to values count.
    /// Some views have that restriction.
    CategoriesCountDoesntEqual,

    /// Count of categories on scale is less than values count.
    /// Some views have that restriction.
    CategoriesCountIsLess,

    /// Provided dataset is empty.
    DataIsEmpty,

    /// Could not save file.
    SaveFileError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::CategoriesCountDoesntEqual => "categories count doesn't equal to data elements count and it's not supported for the selected view".to_string().fmt(f),
            Error::CategoriesCountIsLess => "categories count is less than data elements count and it's not supported for the selected view".to_string().fmt(f),
            Error::DataIsEmpty => "provided data vector is empty".to_string().fmt(f),
            Error::SaveFileError(err) => format!("failed to save file, error: {}", err).fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::SaveFileError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_error() {
        #[derive(Debug)]
        struct B(Option<Box<dyn std::error::Error + 'static>>);

        impl std::fmt::Display for B {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "B")
            }
        }

        impl std::error::Error for B {}

        let err = B(Some(Box::new(Error::DataIsEmpty)));

        let _err = &err as &(dyn std::error::Error);
    }
}
