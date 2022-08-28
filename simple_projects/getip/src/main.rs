
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let res = getip::Config::new(&args);
    let config = res.unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    getip::run(&config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
}
