mod file_io;
mod print;

fn main() {
    let paths = file_io::read_dir(".");
    for path in paths {
        if path.is_dir() {
            println!("\n\nDirectories in this path are:");
            print::display_dir(path);
        }
    }
}
