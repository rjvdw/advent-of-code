use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}

impl Particle {
    pub fn position_at_time(&self, t: i64) -> (i64, i64, i64) {
        let a_t = (t * (t + 1)) / 2;
        (
            self.position.0 + t * self.velocity.0 + a_t * self.acceleration.0,
            self.position.1 + t * self.velocity.1 + a_t * self.acceleration.1,
            self.position.2 + t * self.velocity.2 + a_t * self.acceleration.2,
        )
    }

    // TODO: Implement
    #[allow(clippy::all, dead_code, unused_variables)]
    pub fn will_collide_with(&self, other: &Particle) -> Option<i64> {
        todo!()
    }
}

// TODO: Implement
#[allow(clippy::all, dead_code, unused_variables)]
fn solve((p0, v0, a0): (i64, i64, i64), (p1, v1, a1): (i64, i64, i64)) -> Vec<i64> {
    // f(t) = p + t v + t(t + 1)/2 a
    // we want to solve: p0 + t v0 + t(t + 1)/2 a0 = p1 + t v1 + t(t + 1)/2 a1
    //              <=> (p0 - p1) + t (v0 - v1) + t(t + 1)/2 (a0 - a1) = 0
    //              <=> (a0 - a1)/2 t^2 + ((v0 - v1) - (a0 - a1)/2) t + (p0 - p1) = 0
    //              <=> (a0 - a1) t^2 + (2v0 - 2v1 + a1 - a0) t + (2p0 - 2p1) = 0

    // let:
    let a = a0 - a1;
    let b = 2 * v0 - 2 * v1 + a1 - a0;
    let c = 2 * p0 - 2 * p1;

    // Then we are trying to solve: a t^2 + b t + c = 0
    // First determine the discriminant.
    let d = b * b - 4 * a * c;

    // d = 0 -> no non-integer solutions
    if d < 0 {
        return vec![];
    }

    // TODO, see https://cs.stackexchange.com/a/70458
    todo!()
}

impl Iterator for Particle {
    type Item = (i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;

        Some(self.position)
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
            self.position.0,
            self.position.1,
            self.position.2,
            self.velocity.0,
            self.velocity.1,
            self.velocity.2,
            self.acceleration.0,
            self.acceleration.1,
            self.acceleration.2
        )
    }
}

impl FromStr for Particle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut particle = Particle {
            position: (0, 0, 0),
            velocity: (0, 0, 0),
            acceleration: (0, 0, 0),
        };

        let mut parsing = Parsing::InBetween;
        let mut p = String::new();
        for ch in s.chars() {
            match ch {
                'p' => {
                    parsing = Parsing::Position(0);
                }
                'v' => {
                    parsing = Parsing::Velocity(0);
                }
                'a' => {
                    parsing = Parsing::Acceleration(0);
                }
                '<' | ' ' | '=' => {
                    // noop
                }
                ',' | '>' => match parsing {
                    Parsing::Position(0) => {
                        particle.position.0 = p.parse()?;
                        p.clear();
                        parsing = Parsing::Position(1);
                    }
                    Parsing::Position(1) => {
                        particle.position.1 = p.parse()?;
                        p.clear();
                        parsing = Parsing::Position(2);
                    }
                    Parsing::Position(2) => {
                        particle.position.2 = p.parse()?;
                        p.clear();
                        parsing = Parsing::InBetween;
                    }
                    Parsing::Velocity(0) => {
                        particle.velocity.0 = p.parse()?;
                        p.clear();
                        parsing = Parsing::Velocity(1);
                    }
                    Parsing::Velocity(1) => {
                        particle.velocity.1 = p.parse()?;
                        p.clear();
                        parsing = Parsing::Velocity(2);
                    }
                    Parsing::Velocity(2) => {
                        particle.velocity.2 = p.parse()?;
                        p.clear();
                        parsing = Parsing::InBetween;
                    }
                    Parsing::Acceleration(0) => {
                        particle.acceleration.0 = p.parse()?;
                        p.clear();
                        parsing = Parsing::Acceleration(1);
                    }
                    Parsing::Acceleration(1) => {
                        particle.acceleration.1 = p.parse()?;
                        p.clear();
                        parsing = Parsing::Acceleration(2);
                    }
                    Parsing::Acceleration(2) => {
                        particle.acceleration.2 = p.parse()?;
                        p.clear();
                        parsing = Parsing::InBetween;
                    }
                    Parsing::InBetween => {}
                    _ => unreachable!(),
                },
                _ => {
                    p.push(ch);
                }
            }
        }

        Ok(particle)
    }
}

enum Parsing {
    Position(usize),
    Velocity(usize),
    Acceleration(usize),
    InBetween,
}
