// mod utils;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_string(str: String) -> String {
    let r = String::from("hello ");
    return r + &str;
}

#[wasm_bindgen]
pub fn get_num(num: i32) -> i32 {
    return num;
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Branch {
//     pub name: String,
//     pub node_id: String,
//     pub full_name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Users {
//     pub user: Vec<User>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     pub name: String,
//     pub username: String,
//     pub email: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct DBPedia {
    pub results: Results,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
    pub bindings: Vec<Concepts>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Concepts {
    pub Concept: Values,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Values {
    pub value: String,
}

#[wasm_bindgen]
pub async fn get_data(url_endpoint: String, query_body: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let mut final_query = String::new();

    final_query.push_str("?query=");

    for token in query_body.chars() {
        if token == ' ' {
            final_query.push_str("+");
        } else if token == '?' {
            final_query.push_str("%3F");
        } else if token == '{' {
            final_query.push_str("%7B");
        } else if token == ']' {
            final_query.push_str("%5D");
        } else if token == '[' {
            final_query.push_str("%5B");
        } else if token == '}' {
            final_query.push_str("%7D");
        } else {
            final_query.push(token);
        }
    }

    final_query.push_str("&format=application%2Fsparql-results%2Bjson");

    // let url = format!("https://jsonplaceholder.typicode.com/users?_limit=2");
    // let url = format!("https://api.github.com/repos/SamDz16/webassembly");
    // let url = format!("https://dbpedia.org/sparql?query=select+distinct+%3FConcept+where+%7B%5B%5D+a+%3FConcept%7D+LIMIT+10&format=application%2Fsparql-results%2Bjson&timeout=30000&signal_void=on&signal_unconnected=on");
    let url = format!("{}{}", url_endpoint, final_query);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    // request
    //     .headers()
    //     .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let dbpedia_results: DBPedia = json.into_serde().unwrap();

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&dbpedia_results).unwrap())
}
