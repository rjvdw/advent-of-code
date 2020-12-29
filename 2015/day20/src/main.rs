use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

fn main() {
    let args = get_args(&["<number>"], 1);
    let number = args[1].parse::<usize>().or_exit_with(1);

    let house_number = visit_houses(number, usize::MAX, 10);
    println!(
        "The first house number with at least {} presents is: {}",
        number, house_number
    );

    let house_number = visit_houses(number, 50, 11);
    println!(
        "The first house number with at least {} presents is: {}",
        number, house_number
    );
}

fn visit_houses(nr_houses: usize, limit: usize, nr_presents: usize) -> usize {
    let mut bound = nr_houses / nr_presents;
    let mut houses = vec![0; bound + 1];
    for i in 1.. {
        if i > bound {
            break;
        }
        for j in (i..=bound).step_by(i).take(limit) {
            houses[j] += i * nr_presents;
            if houses[j] >= nr_houses {
                bound = j;
                break;
            }
        }
    }

    houses
        .iter()
        .enumerate()
        .find(|(_, &v)| v >= nr_houses)
        .map(|(j, _)| j)
        .unwrap()
}
