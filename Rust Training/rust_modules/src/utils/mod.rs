use tikv_client::RawClient;


pub async fn get_client() -> RawClient {
    let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();
    client
}


pub async fn add_record<T>(key: String, value: T) where T: ToString {
    let client = get_client().await;
    client.put(key.to_string(), value.to_string()).await.unwrap();

    println!("Record added with Key: {:?}", key);
}


pub async fn get_record(key: String) -> String {
    let client = get_client().await;
    let value = client.get(key.to_string()).await.unwrap().unwrap();

    String::from_utf8(value).unwrap()
}


pub async fn delete_record(key: String) {
    let client = get_client().await;
    client.delete(key.to_string()).await.unwrap();
    
    println!("Record deleted with Key: {:?}", key);
}