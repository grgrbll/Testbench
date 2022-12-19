use std::collections::HashMap;


#[test]
fn http_request() {
    async fn test() -> Result<(), Box<dyn std::error::Error>>{
        let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
    }
    run_async(test);
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[test]
fn http_post() {
    async fn test() -> Result<(), Box<dyn std::error::Error>>{
        let client = reqwest::Client::new();
        let body = "REQUEST_BODY".to_owned();
        let resp = client.post("http://httpbin.org/post")
        .body(body.clone())
        //.form(&[("KEY", "VAL")])
        //.json(&hashmap!("Key" => "Value"))
        .send()
        .await?
        .json::<HashMap<String, serde_json::Value>>()
        .await?;
    assert_eq!(body, resp["data"].as_str().unwrap());
    Ok(())
    }
    run_async(test);
}   

fn run_async<Fut>(f: impl FnOnce() -> Fut)
where
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { f().await.unwrap(); })
}