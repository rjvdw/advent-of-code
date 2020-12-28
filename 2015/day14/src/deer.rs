use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug)]
enum State {
    Resting,
    Traveling,
}

#[derive(Debug)]
pub struct Deer {
    name: String,
    velocity: u32,
    stamina: u32,
    recovery: u32,
    distance_traveled: u32,
    state: State,
    time_in_current_state: u32,
}

impl Deer {
    pub fn travel(&mut self, mut duration: u32) {
        while duration > 0 {
            match self.state {
                State::Resting => {
                    if duration >= self.recovery - self.time_in_current_state {
                        duration -= self.recovery - self.time_in_current_state;
                        self.state = State::Traveling;
                        self.time_in_current_state = 0;
                    } else {
                        self.time_in_current_state += duration;
                        duration = 0;
                    }
                }
                State::Traveling => {
                    if duration >= self.stamina - self.time_in_current_state {
                        duration -= self.stamina - self.time_in_current_state;
                        self.distance_traveled +=
                            self.velocity * (self.stamina - self.time_in_current_state);
                        self.state = State::Resting;
                        self.time_in_current_state = 0;
                    } else {
                        self.distance_traveled += self.velocity * duration;
                        self.time_in_current_state += duration;
                        duration = 0;
                    }
                }
            }
        }
    }

    pub fn get_distance(&self) -> u32 {
        self.distance_traveled
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl FromStr for Deer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = || ParseError(format!("Invalid input: {}", s));

        let mut deer = Deer {
            name: String::new(),
            velocity: 0,
            stamina: 0,
            recovery: 0,
            distance_traveled: 0,
            state: State::Traveling,
            time_in_current_state: 0,
        };

        if let Some(idx) = s.find(' ') {
            deer.name = s[..idx].to_string();
        } else {
            return Err(error());
        }

        if let (Some(start), Some(end)) = (s.find("fly"), s.find("km/s")) {
            deer.velocity = s[start + 3..end].trim().parse::<u32>()?;
        } else {
            return Err(error());
        }

        if let (Some(start), Some(end)) = (s.find("for"), s.find("seconds")) {
            deer.stamina = s[start + 3..end].trim().parse::<u32>()?;
        } else {
            return Err(error());
        }

        if let (Some(start), Some(end)) = (s.rfind("for"), s.rfind("seconds")) {
            deer.recovery = s[start + 3..end].trim().parse::<u32>()?;
        } else {
            return Err(error());
        }

        Ok(deer)
    }
}
