use clap::Parser;

pub mod args;
pub mod repl;

fn main() {
    let args = args::Args::parse();

    match args {
        args::Args::Run(run) => {
            let _ctx = dokki::new_context();
            println!("Running {:?}", run);
        }
        args::Args::Repl(_repl) => repl::ReplContext::default().start(),
        args::Args::PackWasm(pack_wasm) => {
            println!("PackWasm {:?}", pack_wasm);
        }
    }
}
