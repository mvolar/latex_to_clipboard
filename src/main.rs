//extern crate clipboard_win;
//use reqwest::Error;
//use clipboard_win::{formats, get_clipboard, set_clipboard};

// async fn main() {
 

//     let apiId =String::from("W7HK7Y-GK8HGT6289");
//     let input = String::from("pi");
//     println!("{}",apiId);

//     //sending api request
//     let request_url = format!("http://api.wolframalpha.com/v2/query?input=pi&appid={api}",api=apiId);
//     //println!("{}", request_url);
//     //let response = reqwest::get(&request_url);


//     let b = reqwest::get("https://swapi.dev/api/people")
//     .await?
//     .json()
//     .await?;

// println!("Got {:?}", b);
// println!("body = {:?}", body);
//     //set_clipboard(formats::Unicode, text).expect("To set clipboard");
//     //Type is necessary as string can be stored in various storages
//     //let result: String = get_clipboard(formats::Unicode).expect("To set clipboard");
//     //println!("{}",result)
// }


// use serde_json::{self, Value};
// use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub queryresult: Queryresult,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Queryresult {
    pub success: bool,
    pub error: bool,
    pub numpods: i64,
    pub datatypes: String,
    pub timedout: String,
    pub timedoutpods: String,
    pub timing: f64,
    pub parsetiming: f64,
    pub parsetimedout: bool,
    pub recalculate: String,
    pub id: String,
    pub host: String,
    pub server: String,
    pub related: String,
    pub version: String,
    pub inputstring: String,
    pub pods: Vec<Pod>,
   
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    pub title: String,
    pub scanner: String,
    pub id: String,
    pub position: i64,
    pub error: bool,
    pub numsubpods: i64,
    pub primary: bool,
    pub subpods: Vec<Subpod>,
    pub expressiontypes: Expressiontypes,
    
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subpod {
    pub title: String,
    pub plaintext: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Expressiontypes {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub name: String,
    pub input: String,
    pub stepbystep: bool,
}


use reqwest;
use std::error::Error;
use std::time::Duration;
use serde::{Serialize,Deserialize};
use urlencoding::encode;


use clipboard_win::{formats, get_clipboard, set_clipboard};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let client = reqwest::Client::new();
    let clip_string : String = get_clipboard(formats::Unicode).expect("Failed to get clipboard");
    let input : String=format!(r"{}",clip_string);
    //let input = String::from(r#"\binom{10}{2}+\binom{25}{2}"#);
     let apiId =String::from("W7HK7Y-GK8HGT6289");
     let encoded = encode(&input);
        println!("{}", encoded);
     println!("{}",input);
     let request_url = format!(r#"http://api.wolframalpha.com/v2/query?input="{inp}"&appid={api}&includepodid=Result&output=JSON"#,inp=encoded,api=apiId);
    println!("{}",request_url);
     let http_response = reqwest::get(request_url).await?;
     let response = http_response.json::<Root>().await?;
     println!("{:?}",response);
     let mut result = &response.queryresult.pods[0].subpods[0].plaintext;
     println!("{:?}",result);
     set_clipboard(formats::Unicode, result).expect("To set clipboard");
    Ok(())
}