use std::{
    error::Error,
    io::BufReader,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// The input file path
    filename: PathBuf,

    /// The output folder path
    #[arg(default_value = "python_simulator")]
    output: PathBuf,

    /// Don't try to run the simulator
    #[arg(short, long, default_value = "false")]
    no_run: bool,
}

fn run_simulation(folder: &Path, model: &odeir::Model) -> std::io::Result<()> {
    let delta_time = model.meta_data.delta_time.to_string();
    let mut ode_constants: Vec<_> = model.nodes.values()
        .filter_map(|node| match node {
            odeir::Node::Constant { value, .. } => Some(value),
            _ => None,
        })
            .map(ToString::to_string)
        .collect();
    let mut ode_initial: Vec<_> = model.nodes.values()
        .filter_map(|node| match node {
            odeir::Node::Population { initial_population, .. } => Some(initial_population),
            _ => None,
        })
        .map(ToString::to_string)
        .collect();

    ode_initial.insert(0, "--y0".into());
    ode_constants.insert(0, "--ode-params".into());

    std::env::set_current_dir(folder)?;
    dbg!(std::process::Command::new("python3")
        .arg("simulate.py")
        .args(&["--dt", &delta_time])
        .args(&["--tf", "50.0"])
        .args(&["--output", "output.csv"])
        .args(&ode_initial)
        .args(&ode_constants))
    .spawn()
    .unwrap()
    .wait()
    .unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = std::fs::File::open(args.filename).map(BufReader::new)?;
    let model: odeir::Model = serde_json::from_reader(file)?;

    let ode_string = odeir::transformations::r4k::render_ode(&model);
    std::fs::write(args.output.join("ode.py"), ode_string)?;
    if !args.no_run {
        run_simulation(&args.output, &model)?;
    }
    Ok(())
}
