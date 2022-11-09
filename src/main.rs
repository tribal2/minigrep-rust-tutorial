use std::process;

fn main() {
    // Obtenemos argumentos
    let config = minigrep::Config::from_args()
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(1);
        });

    if let Err(e) = minigrep::app(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
