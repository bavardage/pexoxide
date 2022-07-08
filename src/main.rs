use clap::Parser;
use pyo3::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the pex file
    #[clap(short, long, value_parser)]
    pex: String,

    /// A module contained in the pex
    #[clap(short, long)]
    module: String,

    /// The (no-arg) function to invoke
    #[clap(short, long)]
    function: String,
}

fn main() -> PyResult<()> {
    let args = Args::parse();

    Python::with_gil(|py| {
        let pex_bootstrapper = py
            .import("pex.pex_bootstrapper")
            .expect("python module 'pex' must be installed");
        let bootstrap_env = pex_bootstrapper.getattr("bootstrap_pex_env")?;

        bootstrap_env.call1((args.pex,))?;

        let some_method = py.import(&args.module)?.getattr(args.function)?;

        let result = some_method.call0()?;
        println!("Result: {}", result);

        Ok(())
    })
}
