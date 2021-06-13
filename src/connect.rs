#![allow(dead_code)] 

//extern crate odbc;
// Use this crate and set environmet variable RUST_LOG=odbc to see ODBC warnings
//extern crate env_logger;
//extern crate odbc_safe;
mod base_service;
mod company;

use odbc::*;
use odbc_safe::AutocommitOn;
use company::*;

#[derive(Debug, Clone, Copy)]
struct Point
{
    x : i32,
    y : i32
}

#[derive(Debug, Clone, Copy)]
struct Line
{
    start : Point,
    end   : Point
}

impl Line
{
    fn make_line(start : Point, end : Point) -> Self
    {
        Line{
            start,
            end
        }
    }

    fn test() -> std::result::Result<Environment<Version3>, Option<DiagnosticRecord>>
    {
         let env = create_environment_v3_with_os_db_encoding("GB18030","GB18030").map_err(|e| e.unwrap())?;
         Ok(env)
    }

    fn get_end_point<'a>(&'a self) -> &'a Point
    {
        let _p = Point{
            x : 1,
            y : 1
        };
        &self.start
    }
}


fn test(a : &mut i32) -> (Point,Line)
{
    *a = 2;
    println!("a:{}", a);

    let p1 = Point{
        x : 1,
        y : 1
    };
    let p2 = Point{
        x : 2,
        y : 2
    };
    (p1, Line{start: p1, end: p2})
}

fn main() {  
    let mut env =  create_environment_v3_with_os_db_encoding("GB18030","GB18030").map_err(|e| e.unwrap()).unwrap();        
    let conn = base_service::get_connect(&mut env).unwrap();
    let mut company = company::Company::new();
    company.entity.insert();
    match company.save(&conn)
    {
        Ok(_)=>{println!("新增保存成功。")}
        Err(err)=>{println!("{}", err)}
    }
    
    match Company::open(&conn, &company.company_code)
    {
        Ok(com)=>{
            println!("company:{:?}", com);
            let mut company = com;        

            match company.save(&conn)
            {
                Ok(company)=>{
                    println!("company:{:?}", company);        
                }
                Err(err)=>{
                    println!("{}", err)
                }
            }
        }
        Err(err)=>{
            println!("{}", err)
        }
    }

    company.entity.delete();
    match company.save(&conn)
    {
        Ok(com)=>{
            println!("company:{:?}", com);            
        }
        Err(err)=>{
            println!("{}", err)
        }
    }    

}


fn main2() {
    let mut a = 1;
    let (p, l) = test(&mut a);
    println!("main a:{}", a);
    println!("point:{:?} line:{:?}", p, l);
    let line = Line::make_line(Point{x: 1, y : 1}, Point{x: 2, y: 2});
    println!("line:{:?}", line);   
   
    let mut env =  create_environment_v3_with_os_db_encoding("GB18030","GB18030").map_err(|e| e.unwrap()).unwrap();        
    //let conn_str = String::from("driver={sql server};server=192.168.1.8;database=unitsofterp_dev;uid=main;pwd=unitsoft_main;");
    //let conn = env.connect_with_connection_string(&conn_str).unwrap();    
    //let mut env : std::result::Result<Environment<Version3>, Option<DiagnosticRecord>>
    
    let conn = base_service::get_connect(&mut env).unwrap();
    exec(&conn, &String::from("update company set companynamecn = 'xxxxxxxxx' where companyid = 1"));
    
}

fn exec(conn: &Connection<AutocommitOn>, sql: &String) {
    let stmt = Statement::with_parent(conn).unwrap();
    let rs = stmt.exec_direct(sql).unwrap();
    match rs {
        Data(stmt) => {
            let row_count = stmt.affected_row_count().unwrap();
            println!("Has data and affected row count for last statement: {:?}", row_count);
        },
        NoData(stmt) => {
            let row_count = stmt.affected_row_count().unwrap();
            println!("No data and affected row count for last statement: {:?}", row_count);
        }
    }
}