extern crate built;

fn main() {
    built::write_built_file().expect("Failed to gather compile information");
}
