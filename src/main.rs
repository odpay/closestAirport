use reqwest::Error;
use serde_json::{Value};

static SEARCH_RADIUS: i32 = 1000000; // metres

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ip_address = "";

    let ip_url = format!("https://freeipapi.com/api/json/{}", ip_address);
    let ip_response = reqwest::get(&ip_url).await?.text().await?;
    let ip_parsed: Value = serde_json::from_str(&ip_response).unwrap();
    let longitude = ip_parsed.get("longitude").unwrap();
    let latitude = ip_parsed.get("latitude").unwrap();

    let port_url = format!("https://port-api.com/airport/near/{}/{}?search_radius={}&airport_size=large_airport", longitude, latitude, SEARCH_RADIUS);
    let port_response = reqwest::get(&port_url).await?.text().await?;
    let port_parsed: Value = serde_json::from_str(&port_response).unwrap();
    let features: &Value = port_parsed.get("features").unwrap();

    let mut iata = "NONE".to_string();
    let mut distance = -1.00;
    for feature in features.as_array().unwrap() {
        let properties = feature.get("properties").unwrap().clone();
        let newDistance = properties.get("distance").unwrap().as_f64().unwrap();
        if distance == -1.00 || newDistance < distance {
            distance = newDistance;
            iata = properties.get("iata").unwrap().as_str().unwrap().parse().unwrap();
        }
    }
    println!("Closest large airport: {}", iata);
    println!("{} metres away", distance);

    Ok(())
}