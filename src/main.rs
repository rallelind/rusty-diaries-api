mod initiate_api;
mod api;
mod models;
mod repository;

use rocket::{launch, Rocket, Build};

#[launch]
fn rocket() -> Rocket<Build> {
    initiate_api::initiate()
}

