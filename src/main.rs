use postrges::{Client,NoTls};
use postrges::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;

#[macro_use]
extern crate serde_derive;

//Model implementation
#[derive(Serialize, Deserialize)]
struct User{
    id: Option<i32>,
    name: String,
    email: String,
}
//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

fn handle_client(mut stream:TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();


    match stream.read(&mut buffer){
        Ok(size)=>{
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());


            let (status_line,content) = match &*request{
                r if r.starts_with("POST /users")=> handle_post_request(r),
                r if r.starts_with("GET /users/")=> handle_get_request(r),
                r if r.starts_with("GET /users/")=> handle_get_all_request(r),
                r if r.starts_with("PUT /users/")=> handle_put_request(r),
                r if r.starts_with("DELETE /users/")=> handle_delete_request(r),
                _ => (NOT_FOUND.to_string()," 404 not found".to_string()),
            };

            stream.write_all(format!("{}{}",status_line,content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}",e)
    }
    
}