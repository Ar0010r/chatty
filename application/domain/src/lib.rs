use fake::{Dummy, Fake, Faker};

pub mod dto;
pub mod entities;
pub mod repositories;
pub mod scope;
pub mod source;

pub struct Factory<T: Dummy<fake::Faker>> {
    _phantom: T,
}

impl<T: Dummy<fake::Faker>> Factory<T> {
    pub fn create(count: i64) -> Vec<T> {
        let mut result = Vec::<T>::new();
        for _ in 0..count {
            result.push(Faker.fake());
        }
        result
    }

    pub fn create_one() -> T {
        Faker.fake()
    }
}
