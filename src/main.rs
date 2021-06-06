mod errors;
mod models;
mod cli;

fn main() {
    cli::run_loop(models::TodoStore::new());
}
