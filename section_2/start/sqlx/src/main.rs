use actix_web::web::Form;
use chrono::{DateTime, Utc};
use fake::{faker::{internet::ar_sa::Username, name::ar_sa::{FirstName, LastName}}, Fake};
use serde::Deserialize;
use sqlx::prelude::FromRow;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = sqlx::postgres::PgPool::connect("postgres://tester:tester@localhost:5432/tester")
        .await
        .unwrap();

    // let result = sqlx::query_as::<_, Profile>("select * from profile where id = $1")
    //     .bind(2)
    //     .fetch_one(&conn)
    //     .await;

    // let result = sqlx::query_as::<_, EntityId>("insert into public.message (created_at, updated_at, body, likes, user_id) values(current_timestamp, current_timestamp, $1, $2, $3) returning id")
    //     .bind("HI AGAIN FUCKER!")
    //     .bind(0)
    //     .bind(2)
    //     .fetch_one(&conn)
    //     .await;




    // Inserting with transaction
    let mut tx = conn
        .begin()
        .await
        .unwrap();

    // NOTE: Transaction needs to be mutable in case it needs to roll back!
    let insert_result = 
        sqlx::query_as::<_, EntityId>("INSERT INTO public.profile (created_at, updated_at, user_name, full_name) VALUES(CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $1, $2) returning id")
        .bind(Username().fake::<String>())
        .bind(format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>()))
        .fetch_one(&mut tx)
        .await;

    let query_result = 
        sqlx::query_as::<_, Profile>("select * from profile where id = $1")
        // .bind(insert_result.unwrap().id)
        .bind(3452345)
        .fetch_one(&mut tx)
        .await;

    // println!("{:?}", query_result.unwrap());
    match query_result {
        Ok(profile) => {
            println!("{:?}", profile);
            _ = tx.commit().await;
        },
        Err(_) => {
            println!("failed!");
            _ = tx.rollback().await;
        }
    }

    Ok(())
}

// NOTE: FromRow allows for direct mapping from the
// db to the custom type.
#[allow(unused)]
#[derive(FromRow, Deserialize, Debug)]
struct Profile {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
}

#[allow(unused)]
#[derive(FromRow, Deserialize, Debug)]
struct EntityId {
    id: i64,
}
