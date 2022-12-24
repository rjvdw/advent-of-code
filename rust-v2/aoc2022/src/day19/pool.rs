use std::cmp::Ordering;
use std::ops::{Add, Sub};

use crate::blueprint::Blueprint;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pool {
    pub resources: Resources,
    pub robots: Resources,
}

impl Pool {
    /// List all possible states that are possible in the next iteration.
    pub fn next_pools(&self, blueprint: &Blueprint) -> Vec<Pool> {
        let gathered = self.robots;
        let mut next = vec![];

        // build a geode robot
        if let Ok(pool) = self.try_build_geode_robot(blueprint) {
            next.push(Pool {
                resources: pool.resources + gathered,
                robots: pool.robots,
            });
        }

        // build an obsidian robot
        if let Ok(pool) = self.try_build_obsidian_robot(blueprint) {
            next.push(Pool {
                resources: pool.resources + gathered,
                robots: pool.robots,
            });
        }

        // build a clay robot
        if let Ok(pool) = self.try_build_clay_robot(blueprint) {
            next.push(Pool {
                resources: pool.resources + gathered,
                robots: pool.robots,
            });
        }

        // build an ore robot
        if let Ok(pool) = self.try_build_ore_robot(blueprint) {
            next.push(Pool {
                resources: pool.resources + gathered,
                robots: pool.robots,
            });
        }

        // do nothing
        next.push(Pool {
            resources: self.resources + gathered,
            robots: self.robots,
        });

        next
    }

    /// Have all robots gather one resource.
    #[cfg(test)]
    fn gather(&self) -> Pool {
        Pool {
            resources: self.resources + self.robots,
            ..*self
        }
    }

    /// Try to build an ore robot or report back how many resources are still required.
    fn try_build_ore_robot(&self, blueprint: &Blueprint) -> Result<Pool, Resources> {
        if self.resources >= blueprint.ore_robot {
            Ok(Pool {
                resources: self.resources - blueprint.ore_robot,
                robots: Resources {
                    ore: self.robots.ore + 1,
                    ..self.robots
                },
            })
        } else {
            Err(blueprint.ore_robot.saturating_sub(self.resources))
        }
    }

    /// Try to build a clay robot or report back how many resources are still required.
    fn try_build_clay_robot(&self, blueprint: &Blueprint) -> Result<Pool, Resources> {
        if self.resources >= blueprint.clay_robot {
            Ok(Pool {
                resources: self.resources - blueprint.clay_robot,
                robots: Resources {
                    clay: self.robots.clay + 1,
                    ..self.robots
                },
            })
        } else {
            Err(blueprint.clay_robot.saturating_sub(self.resources))
        }
    }

    /// Try to build an obsidian robot or report back how many resources are still required.
    fn try_build_obsidian_robot(&self, blueprint: &Blueprint) -> Result<Pool, Resources> {
        if self.resources >= blueprint.obsidian_robot {
            Ok(Pool {
                resources: self.resources - blueprint.obsidian_robot,
                robots: Resources {
                    obsidian: self.robots.obsidian + 1,
                    ..self.robots
                },
            })
        } else {
            Err(blueprint.obsidian_robot.saturating_sub(self.resources))
        }
    }

    /// Try to build a geode robot or report back how many resources are still required.
    fn try_build_geode_robot(&self, blueprint: &Blueprint) -> Result<Pool, Resources> {
        if self.resources >= blueprint.geode_robot {
            Ok(Pool {
                resources: self.resources - blueprint.geode_robot,
                robots: Resources {
                    geode: self.robots.geode + 1,
                    ..self.robots
                },
            })
        } else {
            Err(blueprint.geode_robot.saturating_sub(self.resources))
        }
    }

    /// Compute an upper bound for the number of geodes that could potentially be mined given the current situation.
    pub fn potential(&self, time_left: u32) -> u32 {
        // the number of geodes currently in the pool
        self.resources.geode
            // plus the number of geodes that will still be mined by the robots that are currently built
            + self.robots.geode * time_left
            // plus the number of geodes that could be mined if a new geode robot would be built every minute
            + if time_left > 1 { time_left * (time_left - 1) / 2 } else { 0 }
    }
}

impl Default for Pool {
    fn default() -> Self {
        Pool {
            resources: Resources::default(),
            robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Resources {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,
    pub geode: u32,
}

impl Resources {
    #[cfg(test)]
    pub fn new(ore: u32, clay: u32, obsidian: u32, geode: u32) -> Resources {
        Resources {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    pub fn saturating_sub(self, rhs: Resources) -> Resources {
        Resources {
            ore: self.ore.saturating_sub(rhs.ore),
            clay: self.ore.saturating_sub(rhs.clay),
            obsidian: self.ore.saturating_sub(rhs.obsidian),
            geode: self.ore.saturating_sub(rhs.geode),
        }
    }
}

impl Add<Resources> for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub<Resources> for Resources {
    type Output = Resources;

    fn sub(self, rhs: Resources) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let compares = (
            self.ore.cmp(&other.ore),
            self.clay.cmp(&other.clay),
            self.obsidian.cmp(&other.obsidian),
            self.geode.cmp(&other.geode),
        );

        match compares {
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => {
                Some(Ordering::Equal)
            }
            (
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
            ) => Some(Ordering::Less),
            (
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
            ) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_recipe_1() {
        let blueprint = Blueprint {
            id: 1,
            ore_robot: Resources::new(4, 0, 0, 0),
            clay_robot: Resources::new(2, 0, 0, 0),
            obsidian_robot: Resources::new(3, 14, 0, 0),
            geode_robot: Resources::new(2, 0, 7, 0),
        };
        let pool = Pool::default();

        // minute 1: do nothing
        let pool = pool.gather();

        // minute 2: do nothing
        let pool = pool.gather();

        // minute 3: build a clay-collecting robot
        let pool = pool.gather().try_build_clay_robot(&blueprint).unwrap();

        // minute 4: do nothing
        let pool = pool.gather();

        // minute 5: build a clay-collecting robot
        let pool = pool.gather().try_build_clay_robot(&blueprint).unwrap();

        // minute 6: do nothing
        let pool = pool.gather();

        // minute 7: build a clay-collecting robot
        let pool = pool.gather().try_build_clay_robot(&blueprint).unwrap();

        // minute 8: do nothing
        let pool = pool.gather();

        // minute 9: do nothing
        let pool = pool.gather();

        // minute 10: do nothing
        let pool = pool.gather();

        // minute 11: build a obsidian-collecting robot
        let pool = pool.gather().try_build_obsidian_robot(&blueprint).unwrap();

        // minute 12: build a clay-collecting robot
        let pool = pool.gather().try_build_clay_robot(&blueprint).unwrap();

        // minute 13: do nothing
        let pool = pool.gather();

        // minute 14: do nothing
        let pool = pool.gather();

        // minute 15: build a obsidian-collecting robot
        let pool = pool.gather().try_build_obsidian_robot(&blueprint).unwrap();

        // minute 16: do nothing
        let pool = pool.gather();

        // minute 17: do nothing
        let pool = pool.gather();

        // minute 18: build a geode-cracking robot
        let pool = pool.gather().try_build_geode_robot(&blueprint).unwrap();

        // minute 19: do nothing
        let pool = pool.gather();

        // minute 20: do nothing
        let pool = pool.gather();

        // minute 21: build a geode-cracking robot
        let pool = pool.gather().try_build_geode_robot(&blueprint).unwrap();

        // minute 22: do nothing
        let pool = pool.gather();

        // minute 23: do nothing
        let pool = pool.gather();

        // minute 24: do nothing
        let pool = pool.gather();

        assert_eq!(
            pool,
            Pool {
                resources: Resources::new(6, 41, 8, 9),
                robots: Resources::new(1, 4, 2, 2),
            }
        );
    }

    mod pool {
        use super::*;

        #[test]
        fn test_next_pools_when_no_resources() {
            let pool = Pool {
                resources: Resources::new(0, 0, 0, 0),
                robots: Resources::new(1, 2, 3, 4),
            };

            let next = pool.next_pools(&blueprint());

            assert_eq!(
                next,
                vec![Pool {
                    resources: Resources::new(1, 2, 3, 4),
                    robots: Resources::new(1, 2, 3, 4),
                }]
            );
        }

        #[test]
        fn test_buying_an_ore_robot() {
            let pool = Pool {
                resources: Resources::new(2, 0, 0, 0),
                robots: Resources::new(0, 0, 0, 0),
            };

            let next = pool.next_pools(&blueprint());

            assert_eq!(
                next,
                vec![
                    Pool {
                        resources: Resources::new(0, 0, 0, 0),
                        robots: Resources::new(1, 0, 0, 0),
                    },
                    Pool {
                        resources: Resources::new(2, 0, 0, 0),
                        robots: Resources::new(0, 0, 0, 0),
                    },
                ]
            );
        }

        #[test]
        fn test_buying_multiple_robots() {
            let pool = Pool {
                resources: Resources::new(100, 100, 0, 0),
                robots: Resources::new(0, 0, 0, 0),
            };

            let next = pool.next_pools(&blueprint());

            assert_eq!(
                next,
                vec![
                    Pool {
                        resources: Resources::new(96, 98, 0, 0),
                        robots: Resources::new(0, 0, 1, 0),
                    },
                    Pool {
                        resources: Resources::new(96, 100, 0, 0),
                        robots: Resources::new(0, 1, 0, 0),
                    },
                    Pool {
                        resources: Resources::new(98, 100, 0, 0),
                        robots: Resources::new(1, 0, 0, 0),
                    },
                    Pool {
                        resources: Resources::new(100, 100, 0, 0),
                        robots: Resources::new(0, 0, 0, 0),
                    },
                ]
            );
        }

        #[test]
        fn test_potential_with_no_resources_or_robots() {
            let pool = Pool {
                resources: Resources::new(0, 0, 0, 0),
                robots: Resources::new(1, 0, 0, 0),
            };

            assert_eq!(pool.potential(24), (1..24).sum());
        }

        #[test]
        fn test_potential_with_some_resources_and_some_robots() {
            let pool = Pool {
                resources: Resources::new(0, 0, 0, 5),
                robots: Resources::new(1, 0, 0, 2),
            };

            assert_eq!(pool.potential(24), 5 + 48 + (1..24).sum::<u32>());
        }

        fn blueprint() -> Blueprint {
            Blueprint {
                id: 1,
                ore_robot: Resources::new(2, 0, 0, 0),
                clay_robot: Resources::new(4, 0, 0, 0),
                obsidian_robot: Resources::new(4, 2, 0, 0),
                geode_robot: Resources::new(4, 0, 2, 0),
            }
        }
    }

    mod resources {
        use super::*;

        #[test]
        fn test_add() {
            let r1 = Resources::new(1, 2, 3, 4);
            let r2 = Resources::new(6, 7, 8, 9);
            let r3 = Resources::new(7, 9, 11, 13);

            assert_eq!(r1 + r2, r3);
        }

        #[test]
        fn test_sub() {
            let r1 = Resources::new(7, 9, 11, 13);
            let r2 = Resources::new(6, 7, 8, 9);
            let r3 = Resources::new(1, 2, 3, 4);

            assert_eq!(r1 - r2, r3);
        }

        mod partial_ord {
            use super::*;

            #[test]
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            fn test_le() {
                assert!(Resources::new(1, 1, 1, 1) <= Resources::new(1, 1, 1, 1));
                assert!(Resources::new(1, 1, 1, 1) <= Resources::new(1, 1, 1, 2));
                assert!(Resources::new(1, 1, 1, 1) <= Resources::new(1, 1, 2, 1));
                assert!(Resources::new(1, 1, 1, 1) <= Resources::new(1, 2, 1, 1));
                assert!(Resources::new(1, 1, 1, 1) <= Resources::new(2, 1, 1, 1));
                assert!(Resources::new(1, 1, 1, 1) <= Resources::new(2, 2, 2, 2));
                assert!(!(Resources::new(1, 1, 1, 1) <= Resources::new(1, 1, 1, 0)));
                assert!(!(Resources::new(1, 1, 1, 1) <= Resources::new(1, 1, 0, 1)));
                assert!(!(Resources::new(1, 1, 1, 1) <= Resources::new(1, 0, 1, 1)));
                assert!(!(Resources::new(1, 1, 1, 1) <= Resources::new(0, 1, 1, 1)));
                assert!(!(Resources::new(1, 1, 1, 1) <= Resources::new(0, 0, 0, 0)));
            }

            #[test]
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            fn test_ge() {
                assert!(Resources::new(1, 1, 1, 1) >= Resources::new(1, 1, 1, 1));
                assert!(Resources::new(1, 1, 1, 2) >= Resources::new(1, 1, 1, 1));
                assert!(Resources::new(1, 1, 2, 1) >= Resources::new(1, 1, 1, 1));
                assert!(Resources::new(1, 2, 1, 1) >= Resources::new(1, 1, 1, 1));
                assert!(Resources::new(2, 1, 1, 1) >= Resources::new(1, 1, 1, 1));
                assert!(Resources::new(2, 2, 2, 2) >= Resources::new(1, 1, 1, 1));
                assert!(!(Resources::new(1, 1, 1, 0) >= Resources::new(1, 1, 1, 1)));
                assert!(!(Resources::new(1, 1, 0, 1) >= Resources::new(1, 1, 1, 1)));
                assert!(!(Resources::new(1, 0, 1, 1) >= Resources::new(1, 1, 1, 1)));
                assert!(!(Resources::new(0, 1, 1, 1) >= Resources::new(1, 1, 1, 1)));
                assert!(!(Resources::new(0, 0, 0, 0) >= Resources::new(1, 1, 1, 1)));
            }
        }
    }
}
