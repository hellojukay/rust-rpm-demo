extern crate clap;
use clap::{Arg, App as CliApp};

use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use std::fs::File;
use std::process;
use std::io::*;


async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app = CliApp::new("rust-rpm-demo").version("v1.0.0").author("hellojukay")
        .arg(Arg::with_name("bind")
             .short("b")
             .takes_value(true)
             .long("bind")
             .help("bind address: 0.0.0.0:8080"))
        .arg(Arg::with_name("pid-file")
             .short("pid")
             .long("pid-file")
             .takes_value(true)
             .help("pid filename")).get_matches();
    let pid_file = app.value_of("pid-file").unwrap_or("/var/run/rust-rpm-demo.pid");
    let bind = app.value_of("bind").unwrap_or("0.0.0.0:8080");
    write_pid(pid_file.to_string());
    println!("write pid to file: {}",pid_file);
    println!("binding on address: {}",bind);

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(bind)?
    .run()
    .await
}
fn write_pid(pid_file: String) {
    let mut file = match File::create(pid_file) {
        Ok(f) => f,
        Err(why) => panic!("can not create file {}",why),
    };
    writeln!(file,"{}",process::id());
    file.sync_data();
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }
}
