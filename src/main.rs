use std::env;

fn main() {
    let args: String = env::args().skip(1).collect();
    if args.is_empty() {
        println!("\nWelcome to rcalc!\nYou can evaluate math expressions using + - * / % ^ ()\n");
        rcalc::run();
    } else {
        rcalc::compute(&args);
    }
}
