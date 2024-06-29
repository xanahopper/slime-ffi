pub trait FromWith<T, R> {
    fn from_with(value: T, rt: R) -> Self;
}

pub trait IntoWith<T, R> {
    fn into_with(self, rt: R) -> T;
}

pub trait TryFromWith<T, R>: Sized {
    type Error;

    fn try_from_with(value: T, rt: R) -> Result<Self, Self::Error>;
}

pub trait TryIntoWith<T, R> {
    type Error;

    fn try_into_with(self, rt: R) -> Result<T, Self::Error>;
}

impl<T, R> FromWith<T, ()> for R
where
    R: From<T>,
{
    fn from_with(value: T, rt: ()) -> Self {
        R::from(value)
    }
}

impl<T, R, E> TryFromWith<T, ()> for R
where
    R: TryFrom<T, Error = E>,
{
    type Error = E;

    fn try_from_with(value: T, rt: ()) -> Result<Self, Self::Error> {
        R::try_from(value)
    }
}

impl<T, R, RT> IntoWith<R, RT> for T
where
    R: FromWith<T, RT>,
{
    fn into_with(self, rt: RT) -> R {
        R::from_with(value, rt)
    }
}

impl<T, R, RT, E> TryIntoWith<R, RT> for T
where
    R: TryFromWith<T, RT, Error = E>,
{
    type Error = E;

    fn try_into_with(self, rt: RT) -> Result<R, Self::Error> {
        R::try_from_with(value, rt)
    }
}

