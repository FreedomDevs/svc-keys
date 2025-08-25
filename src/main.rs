mod models;
mod permissions;
mod routes;
mod store;

use axum::{Router, routing::post};
use colored::*;
use permissions::load_permissions_embedded;
use routes::{token, validate};
use std::net::SocketAddr;
use store::new_store;
use token::AppState;

fn print_banner(service_name: &str, version: &str, addr: &str) {
    let width = 30;

    let top = format!("╭{}╮", "─".repeat(width));
    let bottom = format!("╰{}╯", "─".repeat(width));

    println!("{}", top);

    let title_line = format!("{} {} ", service_name, version);
    let padding = (width - title_line.len()) / 2;
    print!("│{}{}", " ".repeat(padding), "");
    print_gradient(&title_line);
    println!("{}", " ".repeat(width - padding - title_line.len()) + "│");

    let addr_line = format!("IP: {}", addr);
    let padding = (width - addr_line.len()) / 2;
    println!(
        "│{}{}{}│",
        " ".repeat(padding),
        addr_line,
        " ".repeat(width - padding - addr_line.len())
    );

    println!("{}", bottom);
}

fn print_gradient(text: &str) {
    let gradient = [
        Color::BrightRed,
        Color::BrightYellow,
        Color::BrightGreen,
        Color::BrightCyan,
        Color::BrightBlue,
        Color::BrightMagenta,
    ];
    for (i, c) in text.chars().enumerate() {
        let color = gradient[i % gradient.len()];
        print!("{}", c.to_string().color(color));
    }
}

#[tokio::main]
async fn main() {
    let permissions = load_permissions_embedded();
    let state = AppState {
        tokens: new_store(),
        permissions,
    };

    let app = Router::new()
        .nest(
            "/keys",
            Router::new()
                .route("/token", post(token::issue_token))
                .route("/validate", post(validate::validate_token)),
        )
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    print_banner("svc-keys", "V0.1", &addr.to_string());
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
