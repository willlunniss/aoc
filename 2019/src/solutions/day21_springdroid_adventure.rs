use crate::intcode::Intcode;

struct SpringDroid {
    controller: Intcode,
}

impl SpringDroid {
    pub fn new(program: &str) -> Self {
        Self {
            controller: Intcode::from_with(program, 1024 * 1024),
        }
    }

    /// Surveys the hull by running the supplied springscript
    pub fn survey(&mut self, springscript: &[&str]) -> isize {
        // Load the springscript
        for line in springscript {
            self.controller.inputln(line);
        }
        // Lets go!
        self.controller.run();

        if *self.controller.outputs().back().unwrap() < 256 {
            // Something has gone wrong (final value should be outside of ASCII range)
            // Print contents to help debug
            self.controller.print_outputs_as_ascii();
            return 0;
        }

        // Return hull damage report value
        return self.controller.outputs().pop_back().unwrap();
    }
}

#[aoc(day21, part1)]
fn part1(input: &str) -> isize {
    let mut droid = SpringDroid::new(input);

    // If D is solid and there is a hole at B or C then jump (early jump)
    // OR if there is a hole at A  then jump (must jump)
    let springscript = [
        "NOT B J",
        "NOT C T",
        "OR T J",  // Hole at either B or C
        "AND D J", // AND no hole at D
        "NOT A T",
        "OR T J", // OR hole at A
        "WALK"
        ];

    droid.survey(&springscript)
}

#[aoc(day21, part2)]
fn part2(input: &str) -> isize {
    let mut droid = SpringDroid::new(input);

    // If D and H (to allow double jump if needed) are solid
    // AND there is a hole at B or C then jump (early jump)
    // OR if there is a hole at A  then jump (must jump)
    let springscript = [
        "NOT B J",
        "NOT C T",
        "OR T J",  // Hole at either B or C
        "AND D J", 
        "AND H J", // AND no hole at D or H
        "NOT A T",
        "OR T J", // OR hole at A
        "RUN"
        ];

    droid.survey(&springscript)
}
