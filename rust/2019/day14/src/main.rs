use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::reaction::Reaction;

mod reaction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let reactions: Vec<Reaction> = File::open(&args[1]).read_lines(1).collect();
    let map = create_map(&reactions);

    println!(
        "You will need {} ORE to produce 1 FUEL.",
        find_minimum_required_quantity(&map, &mut HashMap::new(), "ORE", "FUEL", 1)
    );

    println!(
        "If you have 1000000000000 ORE, you can produce {} FUEL.",
        find_maximum_producible_quantity(&map, "ORE", 1000000000000, "FUEL"),
    )
}

fn create_map(reactions: &[Reaction]) -> HashMap<String, Reaction> {
    let mut map = HashMap::new();
    for reaction in reactions {
        map.insert(reaction.output_chemical.to_string(), reaction.clone());
    }
    map
}

fn find_minimum_required_quantity(
    reactions: &HashMap<String, Reaction>,
    surplus: &mut HashMap<String, usize>,
    source_chemical: &str,
    requested_chemical: &str,
    requested_quantity: usize,
) -> usize {
    if source_chemical == requested_chemical {
        if let Some(sp) = surplus.get_mut(source_chemical) {
            if *sp >= requested_quantity {
                *sp -= requested_quantity;
                0
            } else {
                let required_target_quantity = requested_quantity - *sp;
                *sp = 0;
                required_target_quantity
            }
        } else {
            requested_quantity
        }
    } else {
        let reaction = reactions.get(requested_chemical).unwrap().clone();

        // factor is how often we will need to run the reaction in order to produce at least the
        // requested quantity
        let factor = 1 + (requested_quantity - 1) / reaction.output_quantity;

        let mut required_target_quantity = 0;
        for (input_quantity, input_chemical) in reaction.input {
            let mut needed = input_quantity * factor;

            // if we still have some in surplus, get it from there first
            if let Some(sp) = surplus.get_mut(&input_chemical) {
                if *sp >= needed {
                    *sp -= needed;
                    needed = 0;
                } else {
                    needed -= *sp;
                    *sp = 0;
                }
            }

            // if we still need more, produce it
            if needed > 0 {
                required_target_quantity += find_minimum_required_quantity(
                    reactions,
                    surplus,
                    source_chemical,
                    &input_chemical,
                    needed,
                );
            }
        }

        // anything left after the reaction goes into the surplus
        surplus.insert(
            requested_chemical.to_string(),
            factor * reaction.output_quantity - requested_quantity,
        );

        required_target_quantity
    }
}

fn find_maximum_producible_quantity(
    reactions: &HashMap<String, Reaction>,
    source_chemical: &str,
    source_chemical_surplus: usize,
    requested_chemical: &str,
) -> usize {
    // first determine how much of the source chemical we need per unit of the requested chemical
    let source_requested_ratio = find_minimum_required_quantity(
        reactions,
        &mut HashMap::new(),
        source_chemical,
        requested_chemical,
        1,
    );

    // put the surplus of the source chemical in the stockpile
    let mut surplus = HashMap::new();
    surplus.insert(source_chemical.to_string(), source_chemical_surplus);

    let mut produced_quantity = 0;

    // we start by doing batches until we have insufficient surplus of the source material
    while *surplus.get(source_chemical).unwrap() > source_requested_ratio {
        let produced_in_this_batch =
            *surplus.get(source_chemical).unwrap() / source_requested_ratio;

        find_minimum_required_quantity(
            reactions,
            &mut surplus,
            source_chemical,
            requested_chemical,
            produced_in_this_batch,
        );

        produced_quantity += produced_in_this_batch;
    }

    // now just keep going until our stockpile is empty, and we can no longer create any more of the
    // requested chemical without additional units of the source chemical
    loop {
        let additional_required = find_minimum_required_quantity(
            reactions,
            &mut surplus,
            source_chemical,
            requested_chemical,
            1,
        );
        if additional_required > 0 {
            return produced_quantity;
        } else {
            produced_quantity += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_minimum_amount_of_ore_1() {
        let reactions = vec![
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_minimum_required_quantity(&map, &mut HashMap::new(), "ORE", "FUEL", 1),
            31
        );
    }

    #[test]
    fn test_minimum_amount_of_ore_2() {
        let reactions = vec![
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_minimum_required_quantity(&map, &mut HashMap::new(), "ORE", "FUEL", 1),
            165
        );
    }

    #[test]
    fn test_minimum_amount_of_ore_3() {
        let reactions = vec![
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_minimum_required_quantity(&map, &mut HashMap::new(), "ORE", "FUEL", 1),
            13312
        );
    }

    #[test]
    fn test_minimum_amount_of_ore_4() {
        let reactions = vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_minimum_required_quantity(&map, &mut HashMap::new(), "ORE", "FUEL", 1),
            180697
        );
    }

    #[test]
    fn test_minimum_amount_of_ore_5() {
        let reactions = vec![
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_minimum_required_quantity(&map, &mut HashMap::new(), "ORE", "FUEL", 1),
            2210736
        );
    }

    #[test]
    fn test_maximum_amount_of_fuel_1() {
        let reactions = vec![
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_maximum_producible_quantity(&map, "ORE", 1_000_000_000_000, "FUEL"),
            82892753
        );
    }

    #[test]
    fn test_maximum_amount_of_fuel_2() {
        let reactions = vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_maximum_producible_quantity(&map, "ORE", 1_000_000_000_000, "FUEL"),
            5586022
        );
    }

    #[test]
    fn test_maximum_amount_of_fuel_3() {
        let reactions = vec![
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ]
        .as_records()
        .unwrap();
        let map = create_map(&reactions);

        assert_eq!(
            find_maximum_producible_quantity(&map, "ORE", 1_000_000_000_000, "FUEL"),
            460664
        );
    }
}
