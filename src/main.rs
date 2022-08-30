use zfx_michelson::michelson::Parser;

#[tokio::main]
async fn main() {
    let mut p = Parser::new();
    let r1 = p.encode().await;
    println!("encoded: {:?}", r1);
    let r2 = p.decode().await;
    println!("decoded: {:?}", r2);
    println!("end");
}
