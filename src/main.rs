use argh::FromArgs;
mod parse;
mod render;

/// Renders Icon Composer files
#[derive(FromArgs)]
struct Args {
    /// input path
    #[argh(positional)]
    input: String,

    /// output path
    #[argh(option, short = 'o')]
    output: Option<String>,

    /// background path
    #[argh(option, short = 'b')]
    background: Option<String>,

    /// print parsed tree
    #[argh(switch, short = 'p')]
    print_tree: bool,
}

fn main() {
    let args: Args = argh::from_env();
    let icon = parse::parse(&args.input, args.background);
    if args.print_tree {
        println!("{:#?}", icon);
    }
    if let Some(output) = args.output {
        render::render(icon, output);
    }
}
