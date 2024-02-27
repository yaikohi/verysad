use crate::{AppState, Dog, TEMPLATES};
use actix_web::{get, Responder};

#[get("/")]
async fn index(state: actix_web::web::Data<AppState>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert(
        "counter",
        &*state.counter.lock().unwrap(),
    );

    let rendered = TEMPLATES
        .render("index.html", &ctx)
        .unwrap();

    actix_web::HttpResponse::Ok().body(rendered)
}

#[get("/counter/{id}/{action}")]
async fn counter_handler(
    action: actix_web::web::Path<(String, String)>,
    state: actix_web::web::Data<AppState>,
) -> impl Responder {
    let (id, action) = action.into_inner();

    let mut ctx = tera::Context::new();

    match action.as_str() {
        "increment" => {
            state.increment(&id);
        }
        "decrement" => {
            state.decrement(&id);
        }
        _ => {
            return actix_web::HttpResponse::BadRequest()
                .body("Invalid action");
        }
    }

    let counter = state.counter.lock().unwrap();
    let count = counter.get(&id).unwrap_or(&0);
    ctx.insert("count", count);
    ctx.insert("id", &id);

    let rendered = TEMPLATES
        .render("counter.html", &ctx)
        .unwrap();

    actix_web::HttpResponse::Ok().body(rendered)
}

#[get("/dogs")]
async fn dogs_handler(
    // state: actix_web::web::Data<AppState>
    ) -> impl Responder {
    let dog_1 = Dog {
        name: "lan".to_string(),
        breed: "labrador".to_string(),
    };

    let dog_2 = Dog {
        name: "bo".to_string(),
        breed: "leonberger".to_string(),
    };

    let dogs = vec![dog_1, dog_2];
    // // state
    // state
    //     .dogs
    //     .lock()
    //     .unwrap()
    //     .insert("2".to_string(), dog_1);

    let mut ctx = tera::Context::new();
    ctx.insert("dogs", &dogs);

    let rendered = TEMPLATES
        .render("dogs.html", &ctx)
        .unwrap();

    actix_web::HttpResponse::Ok().body(rendered)
}
