use std::env;

mod diff;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename1 = args[1].trim().to_string();
    let filename2 = args[2].trim().to_string();

    let lines1 = file::read_file_lines(filename1);
    let lines2 = file::read_file_lines(filename2);

    let grid = diff::diff(&lines1, &lines2, lines1.len(), lines2.len());

    diff::print_diff(grid, &lines1, &lines2, lines1.len(), lines2.len());
}
