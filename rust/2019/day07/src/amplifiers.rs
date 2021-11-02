use shared::intcode;

#[derive(Debug, Clone)]
pub struct Amplifiers {
    programs: Vec<intcode::Program>,
}

impl Amplifiers {
    pub fn new(program: &intcode::Program, count: usize) -> Amplifiers {
        let mut programs = Vec::with_capacity(count);
        for _ in 0..count {
            programs.push(program.clone());
        }
        Amplifiers { programs }
    }

    pub fn run(&mut self, phase_settings: &[i64], mut signal: i64, debugging: bool) -> i64 {
        if phase_settings.len() != self.programs.len() {
            panic!(
                "Cannot run a signal through the amplifiers because the number of phase settings ({}) does not match the number of programs ({}).",
                phase_settings.len(),
                self.programs.len()
            );
        }

        for (idx, phase_setting) in phase_settings.iter().enumerate() {
            self.programs[idx].send_message(*phase_setting);
        }

        let mut programs_available = true;
        while programs_available {
            for program in &mut self.programs {
                program.send_message(signal);
                program.run();
                if debugging || program.has_halted() {
                    programs_available = false;
                }
                signal = program.receive_message().unwrap();
            }
        }
        signal
    }
}
