use std::env;
use std::process;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = args[0].split('/').next_back().unwrap();

    if args.len() != 2 {
        print_usage(prog_name);
    }

    let cols : Vec<&str> = args[1].split(',').collect();
    if cols.len() != 2 {
        print_usage(prog_name);
    }

    let source = usize::from_str(cols[0]);
    let target = usize::from_str(cols[1]);

    if let (Ok(s), Ok(t)) = (source, target) {
        coldiff::column_diff(s, t);
    } else {
        print_usage(prog_name)
    }
}

fn print_usage(prog_name: &str) {
    println!("Usage: {} source_col,target_col
Colors the suffix of target column differing from source column.

source_col and target_col are the column numbers of source and target columns
respectively. Each column is separated by one or more whitespace characters.",
        prog_name);
    process::exit(1);
}
