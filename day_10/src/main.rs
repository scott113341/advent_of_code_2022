mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 10");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2:");
    part_2(get_input())
        .lines()
        .for_each(|l| println!("    {}", l));
}

// Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is
// the sum of these six signal strengths?
fn part_1(cmds: Vec<Cmd>) -> isize {
    let mut cpu = Cpu::new(cmds);
    let mut signal_strengths = vec![];

    // For now, consider the signal strength (the cycle number multiplied by the value of the X
    // register) during the 20th cycle and every 40 cycles after that (that is, during the 20th,
    // 60th, 100th, 140th, 180th, and 220th cycles).
    loop {
        let next_cycle = cpu.cycles() + 1;
        if next_cycle % 40 == 20 {
            signal_strengths.push(next_cycle as isize * cpu.x());
        }

        cpu.run_cycle();

        if cpu.is_done() {
            break;
        }
    }

    signal_strengths.iter().sum()
}

fn part_2(cmds: Vec<Cmd>) -> String {
    let mut cpu = Cpu::new(cmds);
    let mut crt_x_idx: isize = 0;
    let mut crt = String::new();

    // X register controls horizontal position of a sprite, 3 pixels wide. If the sprite is
    // positioned such that one of its three pixels is the pixel currently being drawn, the screen
    // produces a lit pixel (#); otherwise, the screen leaves the pixel dark (.).
    loop {
        let x = cpu.x();
        let sprite_range = (x - 1)..=(x + 1);

        crt.push(if sprite_range.contains(&crt_x_idx) {
            '#'
        } else {
            '.'
        });

        cpu.run_cycle();

        crt_x_idx = (crt_x_idx + 1) % 40;
        if cpu.cycles() % 40 == 0 {
            crt.push('\n');
        }

        if cpu.is_done() {
            break;
        }
    }

    crt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 13140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(get_test_input()),
            r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
            "
            .trim()
            .to_string()
                + "\n"
        );
    }
}
