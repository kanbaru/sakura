use std::{env, sync::Arc};

use cr::CrunchyrollClient;
use harsh::Harsh;
use reqwest::Client as ReqwestClient;
use tokio::sync::RwLock;

extern crate serde_json;

mod cr;
mod db;

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //let mut client = CrunchyrollClient::new_with_vrv_credentials(
    //    ReqwestClient::new(),
    //    "tranlybrandon@yahoo.com".to_string(),
    //    "Demolationer7".to_string(),
    //).await?;

    dotenv::dotenv()?;
    let req_client = ReqwestClient::new();
    let pool = db::get_pool().await?;
    
    let mut client = CrunchyrollClient::new(pool.clone(), req_client).await?;
    match env::var("CR_USERNAME") {
        Ok(_) => {println!("Credentials detected.");
            client
                .add_credentials(env::var("CR_USERNAME")?, env::var("CR_PASSWORD")?)
                .await?;
            
        }
        Err(_) => println!("Credentials not detected."),
    };

    dbg!(&client.token);
    
    let client_cover = Arc::new(tokio::sync::RwLock::new(client));

    //let search_result = &client.search("higehiro".to_string()).await?;

    //let season = &client.seasons(search_result.items[0].items[0].id.to_owned()).await?;

    // let episodes = client.episodes(season.items[0].clone().id).await?;

    //let stream = client.stream(episodes.items[1].clone()).await?;

    let hashid = Harsh::builder()
        .length(5)
        .salt("sakura-app!")
        .build()
        .unwrap(); // pad to length 10
//
    //// start up our web server
    //// dunno if i want this in a separate thread or not
    go(pool, client_cover, hashid).await?;
    Ok(())
}

async fn go(
    pool: sqlx::Pool<sqlx::Postgres>,
    crcl: Arc<RwLock<CrunchyrollClient>>,
    hashid: Harsh,
) -> anyhow::Result<()> {
    rocket::build()
        .manage(pool)
        .manage(crcl)
        .manage(hashid)
        .mount(
            "/api",
            rocket::routes![
                api::index,
                api::get_series,
                api::get_episodes,
                api::show_experience,
                api::search
            ],
        )
        .mount("/api/cr", rocket::routes![api::cr::index_series, api::cr::index_episodes])
        .mount("/api/anilist", rocket::routes![api::anilist::get_metadata])
        .launch()
        .await?;
    Ok(())
}

#[rocket::get("/")]
async fn hello() -> &'static str {
    "hello world!"
}
