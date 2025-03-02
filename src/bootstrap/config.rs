use dotenv::dotenv;

pub fn load() {
    dotenv().ok();
}