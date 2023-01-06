use grep_rust::Config;


fn main() {
    let config = Config::build();
    grep_rust::run(config).expect("Error has happened");
}