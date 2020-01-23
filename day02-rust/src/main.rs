mod computer;

use computer::Computer;

fn part1() -> i32 {
    let mut computer = Computer::new();
    computer.flash("input.txt");

    computer.write(1, 12);
    computer.write(2, 02);

    computer.run();

    computer.read(0)
}

fn part2() -> Result<i32, ()> {
    let target = 19690720;
    let mut computer = Computer::new();

    for noun in 0..99 {
        for verb in 0..99 {
            computer.flash("input.txt");
            computer.write(1, noun);
            computer.write(2, verb);
            computer.run();

            if computer.read(0) == target {
                return Result::Ok(noun * 100 + verb)
            }
        }
    }

    return Result::Err(())
}

fn main() {
    println!("part 1 result = {}", part1());
    println!("part 2 result = {}", part2().expect("Answer not found"));
}
