use rocket::{fairing::AdHoc, Build, Rocket};

use rocket_sync_db_pools::diesel;

#[database("main")]
struct Db(diesel::SqliteConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
	// This macro from `diesel_migrations` defines an `embedded_migrations`
	// module containing a function named `run` that runs the migrations in the
	// specified directory, initializing the database.
	embed_migrations!("migrations");

	let conn = Db::get_one(&rocket).await.expect("database connection");
	conn.run(|c| embedded_migrations::run(c))
		.await
		.expect("diesel migrations");

	rocket
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
		rocket
			.attach(Db::fairing())
			.attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
	})
}
