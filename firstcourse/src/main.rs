mod firstmodule;
mod secondmodule;

fn main() {
    firstmodule::print_chars();
    println!();
    secondmodule::exec::print_chars();
}
