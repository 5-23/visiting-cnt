use std::env::var;
use mongodb::{bson::doc, Client, options::{FindOptions, ClientOptions, ServerApi, ServerApiVersion}};
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Count{
    id: isize,
    cnt: usize,
    nick: String
}
pub async fn count() -> &'static str {
    let mongo_key = var("MONGO_KEY").unwrap();

    let mut client_options =
    ClientOptions::parse(mongo_key)
        .await.expect("option error");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).expect("option error");
    
    let db = client.database("counter");
    let collection = db.collection::<Count>("counter");
    collection.insert_one(Count{
        id: 1,
        cnt: 2,
        nick: "5-23".to_string()
    }, None).await.unwrap();
    // println!("{a}");
    "a"
}

pub async fn gen_image() -> &'static str {
    let mongo_key = var("MONGO_KEY").unwrap();

    let mut client_options =
    ClientOptions::parse(mongo_key)
        .await.expect("option error");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).expect("option error");
    
    let db = client.database("counter");
    let collection = db.collection::<Count>("counter");
    let a = collection.find_one(None, None).await.unwrap();
    println!("{:?}", a);
    // collection.insert_one(Count{
    //     id: id,
    //     cnt: 2,
    //     nick: "5-23".to_string()
    // }, None).await.unwrap();
    // id.to_string()
    "a"
}
