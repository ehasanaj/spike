use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    m: String,
}

pub struct CommandLineArgs {
    message: String,
}

impl CommandLineArgs {
    pub fn parse() -> Self {
        let args = Args::parse();
        let message = args.m;
        Self { message }
    }

    pub fn get_message(&self) -> String {
        self.message.to_owned()
    }
}
