#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::env;
    use actix_files::Files;
    use actix_web::*;
    use gressus::backend::database::db_requests::{add_user};
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use surrealdb::Surreal;
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::auth::Root;
    use gressus::app::*;

    // Connect to the database server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

    // Signin to database
    db.signin(Root {
        username: &env::var("DATABASE_USER").unwrap(),
        password: &env::var("DATABASE_PASS").unwrap(),
    })
    .await.unwrap();

    // Select a specific namespace / database
    db.use_ns("main").use_db("main").await.unwrap();

    add_user(&db, "micha".to_string(), "pass".as_bytes()).await;
    add_user(&db, "heiko".to_string(), "pass".as_bytes()).await;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
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
