use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

fn main() {
    let args = get_args(&["<nr elves>"], 1);
    let nr_elves = args[1].parse::<usize>().or_exit_with(1);

    println!(
        "The winner of game 1 with {} elves, is elf #{}.",
        nr_elves,
        play_game_1(nr_elves)
    );
    println!(
        "The winner of game 2 with {} elves, is elf #{}.",
        nr_elves,
        play_game_2(nr_elves)
    );
}

fn play_game_1(size: usize) -> usize {
    let mut circle = vec![0; size];
    for (i, elf) in circle.iter_mut().enumerate().take(size - 1) {
        *elf = i + 1;
    }
    let mut elf = 0;
    while elf != circle[elf] {
        let eliminated = circle[elf];
        circle[elf] = circle[eliminated];
        elf = circle[elf];
    }
    elf + 1
}

fn play_game_2(mut size: usize) -> usize {
    let mut circle = vec![0; size];
    for (i, elf) in circle.iter_mut().enumerate().take(size - 1) {
        *elf = i + 1;
    }
    let mut elf = 0;
    for _ in 0..size / 2 - 1 {
        elf = circle[elf];
    }
    while elf != circle[elf] {
        let eliminated = circle[elf];
        circle[elf] = circle[eliminated];
        size -= 1;
        if size % 2 == 0 {
            elf = circle[elf];
        }
    }
    elf + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game_1() {
        assert_eq!(play_game_1(5), 3);
    }

    #[test]
    fn test_play_game_2() {
        assert_eq!(play_game_2(14), 5);
    }
}
