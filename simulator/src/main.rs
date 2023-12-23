use std::{
    error::Error,
    io::BufReader,
    path::{Path, PathBuf},
};

use odeir::models::{ode::OdeModel as Model, Argument};

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

fn real_path(name: impl AsRef<Path>) -> PathBuf {
    name.as_ref().canonicalize().unwrap()
}

fn run_simulation(folder: &Path, model: &Model) -> std::io::Result<()> {
    let delta_time = model.metadata.delta_time.to_string();
    let mut ode_constants: Vec<_> = model
        .get_constants()
        .filter_map(|c| match c {
            Argument::Value { value, .. } => Some(value),
            _ => None,
        })
        .map(ToString::to_string)
        .collect();
    let (mut ode_initial, components): (Vec<_>, Vec<_>) = model
        .get_populations()
        .filter_map(|pop| match pop {
            Argument::Value { value, name } => Some((value.to_string(), name)),
            _ => None,
        })
        .unzip();

    ode_initial.insert(0, "--y0".into());
    ode_constants.insert(0, "--ode-params".into());

    std::env::set_current_dir(folder)?;
    let mut simulator = std::process::Command::new("python3");
    simulator
        .arg("simulate.py")
        .args(&["--dt", &delta_time])
        .args(&["--tf", &model.metadata.end_time.to_string()])
        .args(&["--output", "output.csv"])
        .args(&ode_initial)
        .args(&ode_constants);

    let mut plotter = std::process::Command::new("python3");
    plotter
        .arg("plot.py")
        .args(&["--input-file", "output.csv"])
        .args(&["--output-file", "output.png"])
        .arg("--components")
        .args(components);

    // Save execute_all.sh
    {
        let prelude = "#!/usr/bin/env sh\ncd $(realpath $0)".to_string();
        let mut commands = vec![prelude];
        commands.extend_from_slice(
            [&simulator, &plotter]
                .map(|i| format!("{:?}", i))
                .as_slice(),
        );

        std::fs::write("execute_all.sh", commands.join("\n"))?;
    }

    for mut p in [simulator, plotter] {
        p.spawn().unwrap().wait().unwrap();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = std::fs::File::open(args.filename).map(BufReader::new)?;
    let model: odeir::json::Model = serde_json::from_reader(file)?;

    let model = match model {
        odeir::json::Model::ODE(model) => model,
        _ => panic!("Only ODEs are supported."),
    };

    let ode_string = odeir::transformations::r4k::render_ode(&model);
    std::fs::write(args.output.join("ode.py"), ode_string)?;
    if !args.no_run {
        run_simulation(&args.output, &model)?;
    }
    Ok(())
}
