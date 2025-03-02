mod bootstrap;
mod state;
mod application;
mod infrastructure;
mod test;
mod common;

#[tokio::main]
async fn main() {
    // Load Config
    bootstrap::config::load();
    // Http
    infrastructure::presenter::http::init().await;
}
