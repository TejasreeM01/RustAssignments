mod errors;
mod logger;
mod models;
mod app;









fn main() {
    println!("Hello, world!");
    let mut app = app::WalletApp::new();
    app.run();
}
