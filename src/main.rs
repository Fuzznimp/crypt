use clap::{App, Arg};

fn main() {
    let matches = App::new("crypt")
        .version("0.1.0")
        .author("Anthony Montaigne. <anthony@montaigne.dev>")
        .about("Simple CLI to create JWS, JWE and JWKS.")
        .arg(
            Arg::new("jws")
                .short('s')
                .long("jws")
                .value_name("SECRET")
                .about("Generate a JWS using a given secret")
                .takes_value(true),
        )
        .get_matches();

    if let Some(i) = matches.value_of("jws") {
        println!("Value for input: {}", i);
    }
}
