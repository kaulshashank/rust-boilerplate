mod file_io;

fn main() {
    let paths = file_io::read_dir(".");
    for path in paths {
        println!("Path is: {}", path.display())
    }
}
