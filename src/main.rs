use std::io;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize,Debug)]
struct WeatherResponse{
    weather:Vec<Weather>,
    main:Main,
    wind:Wind,
    name:String,
    #[serde(rename = "cod")]
    _cod:i32,
}
#[derive(Deserialize,Debug)]
struct Weather{
    description:String,
}
#[derive(Deserialize,Debug)]
struct Main{
    temp:f64,
    humidity:f64,
    pressure:f64
}
#[derive(Debug,Deserialize)]
struct Wind{
    speed:f64,
}

fn get_weather_info(city:&str,country_code:&str,api_key:&str)->Result<WeatherResponse,Box<dyn std::error::Error>>{
    let url:String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",city,country_code,api_key);
    let response = reqwest::blocking::get(&url)?;
    let response_json:WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}
fn disply_weather_info(response:&WeatherResponse){
    let description:&String = &response.weather[0].description;
    let temperature:f64 = response.main.temp;
    let humidity:f64 = response.main.humidity;
    let pressure:f64 = response.main.pressure;
    let wind_speed:f64 = response.wind.speed;
    let weather_text:String = format!(
        "Weather in {} {}
        > Temperature : {:.1}%
        > Humidity : {:.1}%
        > Pressure : {:.1}hPa
        > Wind Speed : {:.1}m/s
        ",
        response.name,
        description,
        temperature,
        humidity,
        pressure, 
        wind_speed

    );
    println!("{}",weather_text);
}

fn main(){
    println!("{}","welcome to weather station".bright_yellow());
    loop{
        println!("{}","please enter the name of the city".bright_green());
        let mut  city = String::new();
        io::stdin().read_line(&mut city).expect("failed to read city");
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("unable to read ");
        let city = city.trim();
        let country_code = country_code.trim();
        let api_key = "1065e306dea4550e1f5e8ab12ffd984a";
        match get_weather_info(city, country_code, api_key) {
            Ok(response)=>{
                disply_weather_info(&response);
            }
            Err(err)=>{
                eprintln!("err{}", err)
            }
        }
    }
}