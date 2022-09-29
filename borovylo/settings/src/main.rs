use std::env;

mod csound;
mod graphql;

fn print_help() {
    eprintln!("Tasks:");
    eprintln!();
    eprintln!("--help          Print this help text");
    eprintln!("csound          Generate csound data tables file csound/variables.csd");
    eprintln!("graphql         Generate rust grapql init file graphql/src/schema/init.rs");
    eprintln!("all             Run csound and graphql task");
}

fn print_unknown(x: &str) {
    eprintln!("cargo settings {} is an invalid command.", x);
    eprintln!();
    eprintln!("Run `cargo settings` for help page.");
}

fn csound() {
    csound::main();
    println!("csound/variables.csd ......... ok");
}

fn graphql() {
    println!("graphql/src/schema/init.rs ... ok");
    graphql::main();
}

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("csound") => csound(),
        Some("graphql") => graphql(),
        Some("all") => {
            graphql();
            csound()
        }
        None | Some("--help") => print_help(),
        Some(x) => print_unknown(x),
    }
}
