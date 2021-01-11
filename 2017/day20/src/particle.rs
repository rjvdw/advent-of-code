use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::math::polynomial::Polynomial;

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

    // TODO: Use
    #[allow(dead_code)]
    pub fn will_collide_with(&self, other: &Particle) -> Option<i64> {
        // Two particles will collide only if at a given time t, their positions are equal. We start
        // by findings times at which the x-coordinate is equal, and then verify if at those times
        // the y and z-coordinates are also equal. To find the times where the x-coordinates are
        // equal, we just need to solve a quadratic equation.
        //
        // The polynomials that describe the x-coordinate take the form:
        //   x(t) = p + t v + t(t + 1)/2 a
        //        = a/2 t^2 + (v + a/2) t + p
        // And since we are only trying to find their roots, we can multiply the entire equation by
        // 2, so we get rid of the divisions:
        //   x(t) = a t^2 + (2v + a) t + 2p

        let y1 = Polynomial::new(&[
            self.acceleration.0,
            2 * self.velocity.0 + self.acceleration.0,
            2 * self.position.0,
        ]);

        let y2 = Polynomial::new(&[
            other.acceleration.0,
            2 * other.velocity.0 + other.acceleration.0,
            2 * other.position.0,
        ]);

        (y1 - y2)
            .find_roots()
            .iter()
            .copied()
            // ignore times in the past
            .filter(|&t| t >= 0)
            // check if the positions actually match
            .filter(|&t| {
                let p0 = self.position_at_time(t);
                let p1 = other.position_at_time(t);

                p0 == p1
            })
            // we only need to consider the first time they collide
            .min()
    }
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
