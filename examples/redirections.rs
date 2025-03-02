use shaddup::run_quietly_with_opts;
pub fn main() {
    println!("Redirecting both");
    run_quietly_with_opts(
        || {
            println!("YOU SHOULD NOT SEE THIS");
            eprintln!("YOU SHOULD NOT SEE THIS");
        },
        shaddup::opts::Opts {
            descriptors: shaddup::opts::Descriptors::StdoutStderr,
        },
    )
    .unwrap();

    println!("Redirecting stdout only");
    run_quietly_with_opts(
        || {
            println!("YOU SHOULD NOT SEE THIS");
            eprintln!("OK");
        },
        shaddup::opts::Opts {
            descriptors: shaddup::opts::Descriptors::Stdout,
        },
    )
    .unwrap();

    println!("Redirecting stderr only");
    run_quietly_with_opts(
        || {
            println!("OK");
            eprintln!("YOU SHOULD NOT SEE THIS");
        },
        shaddup::opts::Opts {
            descriptors: shaddup::opts::Descriptors::Stderr,
        },
    )
    .unwrap();
}
