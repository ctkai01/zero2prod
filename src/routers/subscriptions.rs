use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{query, PgPool};
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // let request_id = Uuid::new_v4();
    // let request_span = tracing::info_span!
    //     ("Adding a new subscriber.", %request_id, subscriber_email = %form.email, subscriber_name=%form.name
    // );

    // let _request_span_guard = request_span.enter();
    // let query_span = tracing::info_span!("Saving new subscriber details in the database");
    // match query!(
    //     r#"
    //         INSERT INTO subscriptions (id, email, name, subscribed_at)
    //         VALUES ($1, $2, $3, $4)
    //     "#,
    //     Uuid::new_v4(),
    //     form.email,
    //     form.name,
    //     Utc::now()
    // )
    // .execute(pool.get_ref())
    // .instrument(query_span)
    // .await
    // {
    //     Ok(_) => {
    //         tracing::info!("New subscriber details have been saved");
    //         HttpResponse::Ok().finish()
    //     }
    //     Err(e) => {
    //         tracing::error!("Failed to execute query: {:?}", e);
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }
        match insert_subscriber(&pool, &form).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish()
        };

    HttpResponse::Ok().finish()
}

pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    tracing::info!("Done Add");

    Ok(())
}
