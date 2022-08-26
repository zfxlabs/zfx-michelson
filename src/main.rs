use zfx_michelson::michelson::Parser;

#[tokio::main]
async fn main() {
    let mut p = Parser::new();
    p.encode().await;
    p.decode().await;
    println!("end");
}
