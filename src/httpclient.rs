extern crate serde;
use serde::{Serialize, Deserialize};
mod company;
use company::*;


#[derive(Serialize)]
struct Body<'a> {
    lang: &'a str,
    body: &'a str,
}

use chrono::prelude::*;
use chrono::Duration;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> 
{       
    let dt = DateTime::parse_from_str(
        "2020-10-20 12:00:00 +0000", "%Y-%m-%d %H:%M:%S %z");
    println!("datetime:{:?}", dt);    
    let tm = FixedOffset::east(0).ymd(2020, 10, 20).and_hms_milli(12, 0, 0, 000);
    println!("tm:{:?}", tm);
    assert_eq!(dt, Ok(FixedOffset::east(0).ymd(2020, 10, 20).and_hms_milli(12, 0, 0, 000)));
        
    let dt = NaiveDateTime::parse_from_str("2020-10-20 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("datetime dt:{:?}", dt); 

    let dt1 = Local.ymd(2020, 10, 12).and_hms(11, 30, 00);
    println!("datetime dt1:{:?} month:{}, day:{}", dt1, dt1.month(), dt1.day()); 

    let client = reqwest::Client::new();    
    for i in 1..=1000{
        let start_time = Local::now();
        let res : serde_json::Value = client.post("http://localhost:8080/company")
            .json(&CompanyInfo{                
                company_name    : "unitsoft".to_string(),
                is_headquater   : true,
                gen_tel         : "021-22817058".to_string(),
                gen_fax         : "021-22817058-8009".to_string(),
                website         : "http://www.unitsoft.com.cn".to_string(),
                office_code     : "SH".to_string(),
                client_type     : "C".to_string()               
            })
            .send()?
            .json()?;
        let end_time = Local::now();
        println!("loop {} current time:{} use time:{} CompanyName:{} website:{}", i, Local::now().format("%Y-%m-%d %H:%M:%S %.3f"), (end_time - start_time).num_milliseconds(),  res["company_name"], res["website"]);            
    }

    std::thread::sleep_ms(100000);
    Ok(())
} 