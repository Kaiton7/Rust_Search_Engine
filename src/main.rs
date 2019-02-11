
extern crate scraper;
extern crate mongodb;
use scraper::Html;

use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;


fn main() {
    let client_RSE = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    //let coll = client.db("test").collection("movies");
    let coll = client_RSE.db("index").collection("url");

    let doc_RSE = doc! {
        "keyword": "today",
        "url": ["https:/local/","https:test"],
    };
    //let doc = doc! {
    //    "title": "Jaws",
    //    "array": [ 1, 2, 3 ],
    //};

    coll.insert_one(doc_RSE.clone(), None)
        .ok().expect("Failed insert");
    // Insert document into 'test.movies' collection
    //coll.insert_one(doc.clone(), None)
    //    .ok().expect("Failed to insert document.");

    let mut cursor_RSE = coll.find(Some(doc_RSE.clone()), None)
        .ok().expect("Failed find");
    // Find the document and receive a cursor
    /* let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");
 */
    let item_RSE = cursor_RSE.next();
    //let item = cursor.next();

    match item_RSE {
        Some(Ok(doc_RSE)) => match doc_RSE.get("keyword") {
            Some(&Bson::String(ref keyword)) => println!("{}", keyword),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
    // cursor.next() returns an Option<Result<Document>>
    /* match item {
        Some(Ok(doc)) => match doc.get("title") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    } */

}
