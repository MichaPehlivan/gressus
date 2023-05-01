use actix_files::Files;
use actix_web::*;
use chrono::Utc;
use gressus::app::*;
use gressus::backend::database::db_requests::{
	add_event, add_task, add_user, change_username, delete_user, get_events, get_tasks,
	user_id_from_name,
};
use gressus::common::model::User;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use std::env;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Datetime, Uuid};

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// Connect to the database server

	use gressus::backend::database::db_requests::add_category;
	use gressus::app::DB;

	let db = &DB;
	db.connect::<Ws>("127.0.0.1:8000").await.unwrap();

	// Signin to database
	db.signin(Root {
		username: &env::var("DATABASE_USER").unwrap(),
		password: &env::var("DATABASE_PASS").unwrap(),
	})
	.await
	.unwrap();

	// Select a specific namespace / database
	db.use_ns("main").use_db("main").await.unwrap();

	add_user(&db, "micha", &"pass".as_bytes().to_vec()).await;
	add_user(&db, "heiko", &"pass".as_bytes().to_vec()).await;
	let micha_id = user_id_from_name(&db, "micha").await.unwrap();
	let heiko_id = user_id_from_name(&db, "heiko").await.unwrap();
	let start = &Datetime::from(Utc::now());
	let end = &Datetime::from(Utc::now());
	add_task(&db, "task1", "test1", start, end, &Uuid::new(), &micha_id).await;
	add_task(&db, "task2", "test2", start, end, &Uuid::new(), &micha_id).await;
	add_task(&db, "task3", "test3", start, end, &Uuid::new(), &heiko_id).await;
	add_task(&db, "task4", "test4", start, end, &Uuid::new(), &heiko_id).await;
	add_event(&db, "event1", "test5", start, end, &Uuid::new(), &micha_id).await;
	add_event(&db, "event2", "test6", start, end, &Uuid::new(), &micha_id).await;
	add_event(&db, "event3", "test7", start, end, &Uuid::new(), &heiko_id).await;
	add_event(&db, "event4", "test8", start, end, &Uuid::new(), &heiko_id).await;
	let micha_tasks = get_tasks(&db, &micha_id).await;
	let heiko_tasks = get_tasks(&db, &heiko_id).await;
	let micha_events = get_events(&db, &micha_id).await;
	let heiko_events = get_events(&db, &heiko_id).await;
	println!("micha's tasks: {:#?}", micha_tasks);
	println!("micha's events: {:#?}", micha_events);
	println!("heiko's tasks: {:#?}", heiko_tasks);
	println!("heiko's events: {:#?}", heiko_events);
	change_username(&db, &micha_id, "michah").await;
	delete_user(&db, &heiko_id).await;
	add_category(&db, &micha_id, &Uuid::new()).await;
	add_category(&db, &micha_id, &Uuid::new()).await;
	let users: Vec<User> = db.select("users").await.unwrap();
	println!("users: {:#?}", users);

	let conf = get_configuration(None).await.unwrap();
	let addr = conf.leptos_options.site_addr;
	// Generate the list of routes in your Leptos App
	let routes = generate_route_list(|cx| view! { cx, <App/> });

	register_server_fns();

	HttpServer::new(move || {
		let leptos_options = &conf.leptos_options;
		let site_root = &leptos_options.site_root;

		App::new()
			.service(actix_web::web::redirect("/", "/month"))
			.route("/api/{tail:.*}", leptos_actix::handle_server_fns())
			.leptos_routes(
				leptos_options.to_owned(),
				routes.to_owned(),
				|cx| view! { cx, <App/> },
			)
			.service(Files::new("/", site_root))
		//.wrap(middleware::Compress::default())
	})
	.bind(&addr)?
	.run()
	.await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	// no client-side main function
	// unless we want this to work with e.g., Trunk for pure client-side testing
	// see lib.rs for hydration function instead
}
