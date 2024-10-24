use clap::Parser;

#[derive(Parser, Debug)]
pub struct Run {
    #[clap(help = "The script to run")]
    scripts: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Repl {
    #[clap(help = "The script to run")]
    scripts: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct PackWasm {
    #[clap(help = "The script to run")]
    scripts: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    #[command(aliases=["r"], about="Run a single script", long_about = None)]
    Run(Run),

    #[command(aliases=["i"], about="Enter to Interactive Mode", long_about = None)]
    Repl(Repl),

    #[command(aliases=["w"], about="Pack the script in a single wasm binary", long_about = None)]
    PackWasm(PackWasm),
}
