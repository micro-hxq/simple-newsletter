#[tokio::main]
async fn main() -> std::io::Result<()> {
    simple_newsletter::run_server("0.0.0.0:8080").await
}
