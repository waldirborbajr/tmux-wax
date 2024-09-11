use rand::Rng;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //if args.len() != 2 {
    //    eprintln!("Usage: {} <tmux_format_string>", args[0]);
    //    std::process::exit(1);
    //}
    //
    //let format_string = &args[1];
    let random_number = rand::thread_rng().gen_range(0..100);
    println!("{}", random_number);
    //
    //println!(
    //    "{}",
    //    format_string.replace("%random_number", &random_number.to_string())
    //);
}
