use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

// use std::io::prelude::*;
// use std::net::TcpStream;
// fn main() -> std::io::Result<()> {
//     let host = "www.google.com";
//     let mut conn = TcpStream::connect(host)?;
//     conn.write_all(b"GET / HTTP/1.0")?;
//     conn.write_all(b"\r\n")?;

//     conn.write_all(b"Host: www.rustinaction.com")?;
//     conn.write_all(b"\r\n\r\n")?; // Two blank new lines signify end of request

//     // std::io::copy() streams bytes from a Reader to a Writer.
//     std::io::copy(&mut conn, &mut std::io::stdout())?;

//     Ok(())
// }
