mod controllers;

use actix_web::{web, HttpServer};
use color_eyre::eyre::Result;
use controllers::controllers::{counter_handler, index};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Template parsing error(s): {e}");
                ::std::process::exit(1);
            }
        }
    };
}

#[derive(Serialize, Deserialize)]
struct AppState {
    counter: Mutex<HashMap<String, i32>>,
}

impl AppState {
    fn increment(&self, id: &str) {
        let mut counter = self.counter.lock().unwrap();

        if counter.contains_key(id) {
            let count = counter.get_mut(id).unwrap();
            *count += 1;
        } else {
            counter.insert(id.to_string(), 0);
        }
    }

    fn decrement(&self, id: &str) {
        let mut counter = self.counter.lock().unwrap();

        if counter.contains_key(id) {
            let count = counter.get_mut(id).unwrap();
            *count -= 1;
        } else {
            counter.insert(id.to_string(), 0);
        }
    }
}

lazy_static! {
    static ref DEFAULT_COUNTERS: Vec<(String, i32)> =
        vec![("c1".to_string(), 0), ("c2".to_string(), 10)];
}

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    HttpServer::new(|| {
        actix_web::App::new()
            // register state
            .app_data(web::Data::new(AppState {
                counter: Mutex::new(
                    DEFAULT_COUNTERS
                        .clone()
                        .into_iter()
                        .collect(),
                ),
            }))
            // index route
            .service(index)
            // counter route
            .service(counter_handler)
            // static files
            .service(
                actix_files::Files::new("/", "./src/static/")
                    .show_files_listing(),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
