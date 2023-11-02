use crate::domain::repositories::TimeRepository;

pub fn init_command(time_repository: Box<dyn TimeRepository>) {
    match time_repository.init_repository() {
        Ok(()) => println!("Successfully initialized database"),
        Err(_) => println!("Failed to initialize database")
    }
}