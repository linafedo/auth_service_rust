pub mod auth;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;







// use rocket::Rocket;
// use rocket::fairing::AdHoc;
// use rocket_contrib::database;
// use rocket_contrib::databases::diesel;
// use rocket_contrib::databases::diesel::PgConnection;
//
// #[database("diesel_postgres_pool")]
// pub struct DatabaseConnection(PgConnection);
//
// embed_migrations!("migrations");
//
// pub trait TimesheetDatabaseInitialized {
//     fn manage_database(self) -> Self;
// }
//
// impl TimesheetDatabaseInitialized for Rocket {
//     fn manage_database(self) -> Self {
//         self.attach(DatabaseConnection::fairing())
//             .attach(AdHoc::on_attach("Running migrations", |r| {
//                 if let Some(c) = DatabaseConnection::get_one(&r) {
//                     if let Err(e) = embedded_migrations::run(&*c) {
//                         eprint!("Failed to run database migrations: {:?}", e);
//                         return Err(r);
//                     }
//                 }
//                 return Ok(r);
//             }))
//     }
// }
//
// pub trait AuthorizationDatabase {
//     fn login(&self, login: &str, password: &str) -> AuthorizationResult;
//     fn registration(&self, login: &str, password: &str) -> RegistrationResult;
// }
//
// enum AuthorizationResult {
//     Ok,
//     NotFound,
//     Other,
// }
//
// enum RegistrationResult {
//     Ok,
//     AlreadyExist,
//     WeakPassword,
//     Other,
// }


