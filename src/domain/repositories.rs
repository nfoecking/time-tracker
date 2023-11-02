pub trait TimeRepository {
    fn init_repository(&self) -> Result<(), TimeRepositoryError>;
}

#[derive(Debug)]
pub enum TimeRepositoryError {
    Connection,
}
