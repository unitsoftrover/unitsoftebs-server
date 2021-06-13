//extern crate serde;
mod company;
mod base_service;

use odbc::*;
use company::*;

use chrono::prelude::*;

fn main()
{
    let env =  create_environment_v3_with_os_db_encoding("GB18030","GB18030").map_err(|e| e.unwrap()).unwrap();        
    let conn = base_service::get_connect(&env).unwrap();

    let first = Local::now();
    let i = 0;
    for i in 0..1000
    {
        let start = Local::now();
        let mut company = Company::new();        
        company.entity.insert();
        let _company1 = company.save(&conn).unwrap();
        let end = Local::now();
        println!("-----第{}轮--当前时间：{}--耗时：{}-------------", i, end.format("%Y-%m-%d %H:%M:%S %.3f"), (end-start).num_milliseconds());
    }
    
    println!("------总记录数{}--总耗时:{}----------", i, (Local::now()-first).num_seconds());

}