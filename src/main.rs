use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use webhook_listener::Config;

#[post("/webhooks/{key}")]
async fn webhook(
    config: web::Data<Config>,
    path: web::Path<String>,
    req_body: String,
) -> impl Responder {
    let key = path.into_inner();

    // check that key is in config.keys
    // if not, return 403
    if !config.keys.contains(&key) {
        return HttpResponse::Forbidden();
    }

    // write req_body to file under ./data/{key}/{timestamp}.json
    // if error, return 500
    let timestamp = time::OffsetDateTime::now_utc().unix_timestamp();
    let filename = format!("./data/{}/{}.json", key, timestamp);
    match std::fs::write(&filename, &req_body) {
        Ok(_) => (),
        Err(err) => {
            println!("Could not write to file {}, got error: {}", filename, err);
            return HttpResponse::InternalServerError();
        }
    }

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = webhook_listener::read_config();

    let bind = (config.clone().host, config.clone().port);

    // check that ./data exists
    // if not, create it
    match std::fs::create_dir("./data") {
        Ok(_) => (),
        Err(err) => {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                println!("Could not create directory ./data, got error: {}", err);
            }
        }
    }

    // check that ./data/{key} exists for each key in config.keys
    // if not, create it
    for key in config.clone().keys {
        let path = format!("./data/{}", key);
        match std::fs::create_dir(&path) {
            Ok(_) => (),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::AlreadyExists {
                    continue;
                }

                println!("Could not create directory {}, got error: {}", path, err);
            }
        }
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(20)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(config.clone()))
            .service(webhook)
    })
    .bind(bind)?
    .run()
    .await
}
