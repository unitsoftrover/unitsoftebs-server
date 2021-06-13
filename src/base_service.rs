// extern crate odbc;
// Use this crate and set environmet variable RUST_LOG=odbc to see ODBC warnings
// extern crate env_logger;
// extern crate odbc_safe;

use odbc::*;
use odbc_safe::AutocommitOn;

pub fn  get_connect<'en>(env : &'en odbc::Environment<odbc_safe::Odbc3>) -> Result<Connection<'en, AutocommitOn> > {      
    let conn_str = String::from("driver={sql server};server=192.168.1.10;database=YONGDA1015;uid=main;pwd=unitsoft_main;");
    let conn = env.connect_with_connection_string(&conn_str)?;
    Ok(conn)
}
