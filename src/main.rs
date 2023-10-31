use std::io::Read;
use serde_json::Value;
use ipinfo::{IpInfo,IpInfoConfig};


const WEATHER_URL:&str = "https://api.open-meteo.com/v1/forecast?latitude=36.9862&longitude=35.3253&current=temperature_2m&daily=temperature_2m_max,temperature_2m_min&timezone=auto&forecast_days=1"; 

fn main() -> Result<(),String> {


    ////1-Get Temperature Of Adana from api of open-meteo website with reqwest

    let mut res = reqwest::blocking::get(WEATHER_URL).map_err(|_|"Couldnot Get URL")?;
    //===The same as below
    // let mut res = match reqwest::blocking::get(WEATHER_URL){
    //     Ok(result) => result,
    //     Err(e) => return Err("Couldnot Get URL".to_string()),
    // };

    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|_|"Couldnot Read")?;
    //body = res.text().map_err(|_|"Couldnot Parse")?; //"res" is moved, so it gives compile error if we want to use res.status()

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);


    ////2-Parse JSON with serde_json
    
    let v:Value = serde_json::from_str(&body).map_err(|_|"Couldnot Deserialize JSON")?;
    let temperature_current = v["current"]["temperature_2m"].as_f64();
    println!("\n{:?}",temperature_current);
    println!("Daily Temperature Max: {}",v["daily"]["temperature_2m_max"][0]);
    println!("Daily Temperature Min: {}",v["daily"]["temperature_2m_min"][0]);
    println!("Current Temperature {}",v["current"]["temperature_2m"]);

    // What is "Value" type of serde_json?
    // enum Value {
    //     Null,
    //     Bool(bool),
    //     Number(Number),
    //     String(String),
    //     Array(Vec<Value>),
    //     Object(Map<String, Value>),
    // }

    
    ////3-Extend Context with if-else
    
    if let Some(temperature) = temperature_current {
        if temperature < 27.0 && temperature > 18.0 {
            println!("The best temperature for ADANA");
        }
        else if temperature > 32.0 {
            println!("Too hot!");
        }
        else{
            println!("Strange!");
        }
    }

    ////4-Find our ip and geographic location(latitude longitude)
    //According to this location, query the weather

    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = find_info_from_remote_ip(); //call async function
    
    let vector_location = rt.block_on(future);//wait until async function completion, //not to wait use spawn

    let api_url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",vector_location[0],vector_location[1]);
    println!("\n{}",api_url);

    let mut res = reqwest::blocking::get(api_url).map_err(|_|"Couldnot Get URL")?;
    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|_|"Couldnot Read")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    let v:Value = serde_json::from_str(&body).map_err(|_|"Couldnot Deserialize JSON")?;
    let temperature_current = v["current"]["temperature_2m"].as_f64();
    println!("\n{:?}",temperature_current);
    println!("Current Temperature {}",v["current"]["temperature_2m"]);
    

    Ok(()) //Result<T,E>
}



//to use await, use async
async fn find_info_from_remote_ip()->Vec<String>{

    let config = IpInfoConfig{
        //Take an token from "ipinfo.io" website
        token : Some("2ad8af72efc6fe".to_string()),
        ..Default::default()
    };

    let mut ipinfo = IpInfo::new(config).expect("should construct");
    let result_ip_details = ipinfo.lookup("176.240.144.149").await; //Check "what is my ip" on internet

    if let Ok(ip_details) = result_ip_details {
    
    println!("\n\n\nip details: {:?}",ip_details);
    let loc_vec:Vec<String> = ip_details.loc.split(',').map(|s|s.to_string()).collect();
    println!("current location of us: {:?}",loc_vec);

    return loc_vec;

    };
    
    let vec:Vec<String> = Vec::new();
    vec
    
}
