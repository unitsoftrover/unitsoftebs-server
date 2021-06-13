extern crate serde;
extern crate odbc;
extern crate odbc_safe;

use odbc::*;
use odbc_safe::AutocommitOn;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entity
{
    pub is_creating : bool,
    pub is_deleted  : bool
}

impl Entity
{
    pub fn insert(&mut self){
        self.is_creating = true;
        self.is_deleted = false;
    }
    pub fn delete(&mut self){
        self.is_creating = false;
        self.is_deleted = true;
    }    
    pub fn save(&mut self){
        self.is_creating = false;
        self.is_deleted = false;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company
{
    pub entity          : Entity,
    pub company_id      : i32,
    pub company_code    : String,
    pub company_name    : String,
    pub is_headquater   : bool,
    pub gen_tel         : String,
    pub gen_fax         : String,
    pub website         : String,
    pub office_code     : String,
    pub client_type     : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompanyInfo
{    
    pub company_name    : String,
    pub is_headquater   : bool,
    pub gen_tel         : String,
    pub gen_fax         : String,
    pub website         : String,
    pub office_code     : String,
    pub client_type     : String   
}

impl Company
{
    pub fn new() -> Company     
    {
        let company = Company{       
            entity : Entity{
                is_creating : false,
                is_deleted  : false
            },       
            company_id      : 1,
            company_code    : "00000000".to_string(),
            company_name    : "unitsoft".to_string(),
            is_headquater   : true,
            gen_tel         : "021-22817058".to_string(),
            gen_fax         : "021-22817058-8009".to_string(),
            website         : "http://www.unitsoft.com.cn".to_string(),
            office_code     : "SH".to_string(),
            client_type     : "C".to_string()               
        };
        company
    }    

    pub fn open(conn : &Connection<AutocommitOn>, company_code : &String) -> std::result::Result<Self, String> 
    {        
        let stmt = Statement::with_parent(conn).unwrap();
        let sql = String::from("select CompanyID, CompanyCode, CompanyName,IsHeadOffice,TelNOOffice,FaxNOOffice, Website, CreateOffice,CompanyType from Company where CompanyCode = '") + company_code.as_str() + "'";
        let rs = stmt.exec_direct(sql.as_str()).unwrap();
        match rs {
            Data(mut stmt) => {
                let row_count = stmt.affected_row_count().unwrap();
                println!("Has data and affected row count for last statement: {:?}", row_count);

                let cols = stmt.num_result_cols().unwrap();
                println!("cols:{}", cols);

                if let Some(mut cursor) = stmt.fetch().unwrap() {
                    let company = Company{
                        entity : Entity{
                            is_creating : false,
                            is_deleted  : false
                        },            
                        company_id      : match cursor.get_data::<i32>(1).unwrap(){Some(val)=>val, None=>0 },
                        company_code    : match cursor.get_data::<String>(2).unwrap(){Some(val)=>val, None=>String::from("")},
                        company_name    : match cursor.get_data::<String>(3).unwrap(){Some(val)=>val, None=>String::from("")},
                        is_headquater   : match cursor.get_data::<bool>(4).unwrap(){Some(val)=>val, None=>false},
                        gen_tel         : match cursor.get_data::<String>(5).unwrap(){Some(val)=>val, None=>String::from("")},
                        gen_fax         : match cursor.get_data::<String>(6).unwrap(){Some(val)=>val, None=>String::from("")},
                        website         : match cursor.get_data::<String>(7).unwrap(){Some(val)=>val, None=>String::from("")},
                        office_code     : match cursor.get_data::<String>(8).unwrap(){Some(val)=>val, None=>String::from("")},
                        client_type     : match cursor.get_data::<String>(9).unwrap(){Some(val)=>val, None=>String::from("")}
                    };
                    println!("Company:{:?}", company);
                    return Ok(company);
                }
                else
                {
                    return Err("获取客户信息失败。".to_string());
                }
            },
            NoData(stmt) => {
                let row_count = stmt.affected_row_count().unwrap();
                println!("No data and affected row count for last statement: {:?}", row_count);
                return Err("客户信息不存在。".to_string());
            }
        }

        // let company = Company{
        //     entity : Entity{
        //         is_creating : false,
        //         is_deleted  : false
        //     },            
        //     company_id      : 1,
        //     company_code    : "C0000001".to_string(),
        //     company_name    : "unitsoft".to_string(),
        //     is_headquater   : true,
        //     gen_tel         : "021-22817058".to_string(),
        //     gen_fax         : "021-22817058-8009".to_string(),
        //     website         : "http://www.unitsoft.com.cn".to_string()            
        // };
        // Ok(company)
    }
    
    pub fn save(&self, conn : &Connection<AutocommitOn>) -> std::result::Result<Company, String> 
    {
        if self.entity.is_creating
        {
            let stmt = Statement::with_parent(conn).unwrap().prepare("
            insert company (CompanyName, IsHeadOffice, TelNOOffice, FaxNOOffice, Website, CreateOffice,CompanyCode, CompanyType) values (?, ?, ?, ?, ?, ?, ?, ?);            
            ").unwrap();
            // select CompanyID, CompanyCode, CompanyName,IsHeadOffice,TelNOOffice,FaxNOOffice, Website, CreateOffice, CompanyType from Company where CompanyID=SCOPE_IDENTITY();
            let stmt = stmt.bind_parameter(1, &self.company_name).unwrap();    
            let stmt = stmt.bind_parameter(2, &self.is_headquater).unwrap();
            let stmt = stmt.bind_parameter(3, &self.gen_tel).unwrap();    
            let stmt = stmt.bind_parameter(4, &self.gen_fax).unwrap();
            let stmt = stmt.bind_parameter(5, &self.website).unwrap();    
            let stmt = stmt.bind_parameter(6, &self.office_code).unwrap();    
            let stmt = stmt.bind_parameter(7, &self.company_code).unwrap();
            let stmt = stmt.bind_parameter(8, &self.client_type).unwrap();            
            let rs = stmt.execute().unwrap();  
           
            if let Data(stmt) = rs
            {
                let row_count = stmt.affected_row_count().unwrap();
                println!("Has data and affected row count for last statement: {:?}", row_count); 
            }
            else
            {
                println!("No data and affected row count for last statement.");
            }            
            
            // match rs    
            // {
            //     Data(stmt) => {
            //         let row_count = stmt.affected_row_count().unwrap();
            //         println!("Has data and affected row count for last statement: {:?}", row_count); 
            //     },
            //     NoData(_) => {                                        
            //         println!("No data and affected row count for last statement.");
            //     }
            // }           
            
            let stmt = Statement::with_parent(&conn).unwrap().prepare("
                        select CompanyID, CompanyCode, CompanyName,IsHeadOffice,TelNOOffice,FaxNOOffice, Website, CreateOffice, CompanyType from Company where CompanyID=IDENT_CURRENT('company');
                    ").unwrap();   
            //let stmt = stmt.bind_parameter(1, &self.company_code).unwrap();                    
            let rs = stmt.execute().unwrap();

            match rs {
                Data(mut stmt) => {
                    let row_count = stmt.affected_row_count().unwrap();
                    println!("Has data and affected row count for last statement: {:?}", row_count);
    
                    let cols = stmt.num_result_cols().unwrap();
                    println!("cols:{}", cols);            

                    if let Some(mut cursor) = stmt.fetch().unwrap() {
                        let mut company = Company{
                            entity : Entity{
                                is_creating : false,
                                is_deleted  : false
                            },            
                            company_id      : match cursor.get_data::<i32>(1).unwrap(){Some(val)=>val, None=>0 },
                            company_code    : match cursor.get_data::<String>(2).unwrap(){Some(val)=>val, None=>String::from("")},
                            company_name    : match cursor.get_data::<String>(3).unwrap(){Some(val)=>val, None=>String::from("")},
                            is_headquater   : match cursor.get_data::<bool>(4).unwrap(){Some(val)=>val, None=>false},
                            gen_tel         : match cursor.get_data::<String>(5).unwrap(){Some(val)=>val, None=>String::from("")},
                            gen_fax         : match cursor.get_data::<String>(6).unwrap(){Some(val)=>val, None=>String::from("")},
                            website         : match cursor.get_data::<String>(7).unwrap(){Some(val)=>val, None=>String::from("")},
                            office_code     : match cursor.get_data::<String>(8).unwrap(){Some(val)=>val, None=>String::from("")},
                            client_type     : match cursor.get_data::<String>(9).unwrap(){Some(val)=>val, None=>String::from("")}
                        };
                        println!("Company:{:?}", company);

                        let stmt = Statement::with_parent(&conn).unwrap().prepare("exec MakeCode 'CompanyCode', ?").unwrap();   
                        let stmt = stmt.bind_parameter(1, &company.company_id).unwrap();                    
                        let rs = stmt.execute().unwrap();
                        if let NoData(stmt) = rs 
                        {
                            let row_count = stmt.affected_row_count().unwrap();
                            println!("新增记录行数: {:?}", row_count);
                        }

                        let stmt = Statement::with_parent(&conn).unwrap().prepare("select CompanyCode from Company where CompanyID=?").unwrap();   
                        let stmt = stmt.bind_parameter(1, &company.company_id).unwrap();                    
                        let rs = stmt.execute().unwrap();
                        if let Data(mut stmt) = rs 
                        {
                            let row_count = stmt.affected_row_count().unwrap();
                            println!("新增客户编码查询行数: {:?}", row_count);
                            
                            if let Some(mut cursor) = stmt.fetch().unwrap() {
                                    company.company_code = match cursor.get_data::<String>(1).unwrap(){Some(val)=>val, None=>String::from("")};
                            }
                        }
                        
                        println!("company:{:?}", company);
                        return Ok(company);
                    }
                    else
                    {
                        return Err("新增客户后获取信息失败。".to_string());
                    }       
                },
                NoData(_) => {                                           
                    return Err("新增客户后获取信息失败。".to_string());    
                }
            }             
        }
        else if self.entity.is_deleted
        {
            let stmt = Statement::with_parent(&conn).unwrap().prepare("delete from company where CompanyCode = ?").unwrap();
            let stmt = stmt.bind_parameter(1, &self.company_code).unwrap();            
            let rs = stmt.execute().unwrap();    
            match rs    
            {
                Data(stmt) => {
                    let row_count = stmt.affected_row_count().unwrap();
                    println!("Has data and affected row count for last statement: {:?}", row_count);

                    let company = Company{       
                        entity : Entity{
                            is_creating : false,
                            is_deleted  : false
                        },       
                        company_id      : 1,
                        company_code    : "C0000001".to_string(),
                        company_name    : "unitsoft".to_string(),
                        is_headquater   : true,
                        gen_tel         : "021-22817058".to_string(),
                        gen_fax         : "021-22817058-8009".to_string(),
                        website         : "http://www.unitsoft.com.cn".to_string(),
                        office_code     : "SH".to_string(),
                        client_type     : "C".to_string()                 
                    };
                    return Ok(company);
                },
                NoData(_) => {           
                    println!("No data and affected row count for last statement.");
                    return Err("删除客户失败。".to_string())                        
                }
            }
        }
        else
        {
            let stmt = Statement::with_parent(&conn).unwrap().prepare("
                update company set CompanyName=?, IsHeadOffice=?, TelNOOffice=?, FaxNOOffice=?, Website=?, CreateOffice=?,CompanyType=?  where CompanyCode = ?;                
                ").unwrap();
            let stmt = stmt.bind_parameter(1, &self.company_name).unwrap();    
            let stmt = stmt.bind_parameter(2, &self.is_headquater).unwrap();
            let stmt = stmt.bind_parameter(3, &self.gen_tel).unwrap();    
            let stmt = stmt.bind_parameter(4, &self.gen_fax).unwrap();
            let stmt = stmt.bind_parameter(5, &self.website).unwrap();    
            let stmt = stmt.bind_parameter(6, &self.office_code).unwrap();
            let stmt = stmt.bind_parameter(7, &self.client_type).unwrap();
            let stmt = stmt.bind_parameter(8, &self.company_code).unwrap();            

            let rs = stmt.execute().unwrap();                
            if let Data(stmt) = rs
            {
                let row_count = stmt.affected_row_count().unwrap();
                println!("Has data and affected row count for last statement: {:?}", row_count);                    
            }
            else
            {
                if let NoData(stmt) = rs
                {
                    let row_count = stmt.affected_row_count().unwrap();
                    println!("Has data and affected row count for last statement: {:?}", row_count);
                    println!("have no result data.")
                }                
            }           

            let stmt = Statement::with_parent(&conn).unwrap().prepare("
                        select CompanyID, CompanyCode, CompanyName,IsHeadOffice,TelNOOffice,FaxNOOffice, Website, CreateOffice, CompanyType from Company where CompanyCode=?;
                    ").unwrap();   
            let stmt = stmt.bind_parameter(1, &self.company_code).unwrap();                    
            let rs = stmt.execute().unwrap();    

            match rs {
                Data(mut stmt) => {
                    let row_count = stmt.affected_row_count().unwrap();
                    println!("Has data and affected row count for last statement: {:?}", row_count);
    
                    let cols = stmt.num_result_cols().unwrap();
                    println!("cols:{}", cols);
    
                    if let Some(mut cursor) = stmt.fetch().unwrap() {
                        let company = Company{
                            entity : Entity{
                                is_creating : false,
                                is_deleted  : false
                            },            
                            company_id      : match cursor.get_data::<i32>(1).unwrap(){Some(val)=>val, None=>0 },
                            company_code    : match cursor.get_data::<String>(2).unwrap(){Some(val)=>val, None=>String::from("")},
                            company_name    : match cursor.get_data::<String>(3).unwrap(){Some(val)=>val, None=>String::from("")},
                            is_headquater   : match cursor.get_data::<bool>(4).unwrap(){Some(val)=>val, None=>false},
                            gen_tel         : match cursor.get_data::<String>(5).unwrap(){Some(val)=>val, None=>String::from("")},
                            gen_fax         : match cursor.get_data::<String>(6).unwrap(){Some(val)=>val, None=>String::from("")},
                            website         : match cursor.get_data::<String>(7).unwrap(){Some(val)=>val, None=>String::from("")},
                            office_code     : match cursor.get_data::<String>(8).unwrap(){Some(val)=>val, None=>String::from("")},
                            client_type     : match cursor.get_data::<String>(9).unwrap(){Some(val)=>val, None=>String::from("")}
                        };
                        println!("Company:{:?}", company);
                        return Ok(company);
                    }
                    else
                    {
                        return Err("更新客户后获取信息失败。".to_string());
                    }
                },
                NoData(_) => {                            
                    return Err("更新客户后获取信息失败。".to_string());    
                }
            }
        }

        // let company = Company{       
        //     entity : Entity{
        //         is_creating : false,
        //         is_deleted  : false
        //     },       
        //     company_id      : 1,
        //     company_code    : "C0000001".to_string(),
        //     company_name    : "unitsoft".to_string(),
        //     is_headquater   : true,
        //     gen_tel         : "021-22817058".to_string(),
        //     gen_fax         : "021-22817058-8009".to_string(),
        //     website         : "http://www.unitsoft.com.cn".to_string(),
        //     office_code     : "SH".to_string(),
        //     client_type     : "C"                           
        // };        
        // return Ok(company);
    }
}
