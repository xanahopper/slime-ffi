pub trait FromWith<T, R> {
    fn from_with(value: T, rt: R) -> Self;
}

pub trait IntoWith<T, R> {
    fn into_with(self, rt: R) -> T;
}

pub trait TryFromWith<T, R> {
    type Error;

    fn try_from_with(value: T, rt: R) -> Result<Self, Self::Error>;
}

pub trait TryIntoWith<T, R> {
    type Error;

    fn try_into_with(self, rt: R) -> Result<T, Self::Error>;
}