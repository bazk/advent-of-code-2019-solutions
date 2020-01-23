mod computer;

use computer::Computer;

fn main() {
    let mut computer = Computer::new();
    computer.flash("input.txt");
    computer.run();
}
