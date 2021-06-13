//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.
//extern crate odbc;
// Use this crate and set environmet variable RUST_LOG=odbc to see ODBC warnings
//extern crate env_logger;
//extern crate odbc_safe;
#![allow(dead_code)] 

use odbc::*;
use actix_web::{get, post, middleware, web, App, Error, HttpResponse, HttpServer};

mod company;
mod base_service;
mod lib;

use company::*;

fn action(str1 : &'static str) -> std::result::Result<String, String>
{   
    Ok(str1.to_string())
}

fn action_add_company(company : web::Json<CompanyInfo>) -> std::result::Result<Company, String>
{    
    let env =  create_environment_v3_with_os_db_encoding("GB18030","GB18030").map_err(|e| e.unwrap()).unwrap();        
    let conn = base_service::get_connect(&env).unwrap();
    let mut company1 = Company{
        entity : Entity{
            is_creating : false,
            is_deleted  : false
        },
        company_id      : 1,
        company_code    : "9999999".to_string(),
        company_name    : company.company_name.clone(),
        is_headquater   : company.is_headquater,
        gen_tel         : company.gen_tel.clone(),
        gen_fax         : company.gen_fax.clone(),
        website         : company.website.clone(),
        office_code     : company.office_code.clone(),
        client_type     : company.client_type.clone()          
    };    
    
    company1.entity.insert();
    let company1 = company1.save(&conn).unwrap();

    Ok(company1)
}


/// Inserts new user with name defined in form.
#[post("/company")]
async fn add_company(
    form: web::Json<CompanyInfo>,
    //form : web::Query<CompanyInfo>
) -> std::result::Result<HttpResponse, Error> {    
    let str1 = lib::hello();
    lib::util::test();
    lib::base::test_base();

    // use web::block to offload blocking Diesel code without blocking server thread
    let company = web::block(move || action_add_company(form))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    // let env =  create_environment_v3_with_os_db_encoding("GB18030","GB18030").map_err(|e| e.unwrap()).unwrap();        
    // let conn = base_service::get_connect(&env).unwrap();
    // let company = Company{
    //     entity : Entity{
    //         is_creating : false,
    //         is_deleted  : false
    //     },
    //     company_id      : 1,
    //     company_code    : "C9999999".to_string(),
    //     company_name    : form.company_name.clone(),
    //     is_headquater   : form.is_headquater,
    //     gen_tel         : form.gen_tel.clone(),
    //     gen_fax         : form.gen_fax.clone(),
    //     website         : form.website.clone(),
    //     office_code     : form.office_code.clone(),
    //     client_type     : form.client_type.clone()          
    // };  
    // company.save(&conn).unwrap();

    println!("company before response:{:?}", company);
    Ok(HttpResponse::Ok().json(company))
} 

/// Finds company by company code.
#[get("/company/{company_code}")]
async fn get_company(    
    company_code: web::Path<String>,
) -> std::result::Result<HttpResponse, Error> {
    
    let company_code = company_code.into_inner();    

    let mut env =  create_environment_v3_with_os_db_encoding("GB18030","GB18030").map_err(|e| e.unwrap()).unwrap();        
    let conn = base_service::get_connect(&mut env).unwrap();
    let company = Company::open(&conn, &company_code);
    match company
    {
        Ok(mut com)=>{
            println!("company:{:?}", com);                  

            match com.save(&conn)
            {
                Ok(company)=>{
                    println!("company:{:?}", company);        
                }
                Err(err)=>{
                    println!("{}", err)
                }
            }

            //use web::block to offload blocking Diesel code without blocking server thread
            let user = web::block(move || action("ok"))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            println!("user:{:?}", user);

            Ok(HttpResponse::Ok().json(com))
        }
        Err(err)=>{
            println!("{}", err);

            let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", company_code));
            Ok(res)
        }
    }

    

}

#[link(name = "cfun", kind = "static")]
extern "C" {
    pub fn say_hello();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    unsafe{say_hello();}

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    //let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .wrap(middleware::Logger::default())
            .service(get_company)           
            .service(add_company)
    })
    .bind(&bind)?
    .run()
    .await
}
