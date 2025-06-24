use builder::{add, models::config::Config};

fn main() {
    println!(
        "\n\n****************************\nStarting the server\n****************************\n\n"
    );

    let config = Config::load("config.toml");

    if let Err(error) = config {
        println!("Loading config Error: {}", error.to_string());
        return;
    }

    let config = config.unwrap();
    println!(
        "{}",
        config.project.max_pending_build
    );

    println!("{}", add(4, 5));
}
