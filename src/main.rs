use clap::{Command, arg, command};

fn main() -> std::io::Result<()> {

    let matches = command!()
        .subcommand(
            Command::new("train")
                .about("Train a new model.")
                .args(
                    [
                        arg!(-c --config <File> "Path to the model training configuration.")
                            .required(true),
                        arg!(-d --dataset <File> "Path to the dataset for the training.")
                            .required(true)
                    ]
                )
        )
        .subcommand(
            Command::new("predict")
                .about("Use a model to do a prediction on a dataset.")
                .args([
                    arg!(-m --model <File> "Path to the folder containing the model.")
                        .required(true),
                    arg!(-d --dataset <File> "Path to the dataset to predict.")
                        .required(true)
                ])
        )
        .subcommand(
            Command::new("compare")
                .about("Compare two training phase of two existing models")
                .args([
                    arg!(--model1 <File> "Path to the folder containing the first model.")
                        .required(true),
                    arg!(--model2 <File> "Path to the folder containing the second model.")
                        .required(true),
                ])
        )
        .subcommand_required(true)
        .get_matches();

    let subcommand = matches
        .subcommand_name()
        .unwrap();

    let ctx = matches.subcommand_matches(subcommand).unwrap();

    match subcommand {
        "train" => {
            let config = ctx.get_one::<String>("config").unwrap();
            let dataset = ctx.get_one::<String>("dataset").unwrap();
            println!("config: {config}");
            println!("dataset: {dataset}");
        },
        "predict" => {
            let dataset = ctx.get_one::<String>("dataset").unwrap();
            let model = ctx.get_one::<String>("model").unwrap();
            println!("dataset: {dataset}");
            println!("model: {model}");
        },
        "compare" => {
            let model1 = ctx.get_one::<String>("model1").unwrap();
            let model2 = ctx.get_one::<String>("model2").unwrap();
            println!("model1: {model1}");
            println!("model2: {model2}");
        },
        _ => unreachable!()
    }

    Ok(())
}
