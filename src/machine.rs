use instruction::Instruction;

pub fn execute_program(program: &[Instruction], input: &[u8]) -> Vec<u8> {
    let mut machine = Machine::new(program, input);
    machine.execute();
    machine.output
}

struct Machine {
    code: Vec<Instruction>,
    data: Vec<u8>,
    input: Vec<u8>,
    output: Vec<u8>,
    ip: usize, // instruction pointer
    dp: usize, // data pointer
}

impl Machine {
    fn new(program: &[Instruction], input: &[u8]) -> Machine {
        // We need to consume the input from first to last byte, but Vec only allows us to pop
        // the last element. Thus, we store the input in reverse.
        let mut input_vec = input.to_vec();
        input_vec.reverse();

        Machine {
            code: program.to_vec(),
            data: vec![0],
            input: input_vec,
            output: Vec::new(),
            ip: 0,
            dp: 0,
        }
    }

    fn execute(&mut self) {
        while self.ip < self.code.len() {
            use instruction::Instruction::*;

            match self.code[self.ip] {
                Next => self.exec_next(),
                Prev => self.exec_prev(),
                Inc => self.exec_inc(),
                Dec => self.exec_dec(),
                Put => self.exec_put(),
                Get => self.exec_get(),
                Skip => self.exec_skip(),
                Loop => self.exec_loop(),
            };

            self.ip += 1;
        }
    }

    // instruction handlers

    fn exec_next(&mut self) {
        self.dp += 1;
        if self.dp == self.data.len() {
            self.data.push(0);
        }
    }

    fn exec_prev(&mut self) {
        if self.dp == 0 {
            panic!("Data pointer moved below 0.");
        }
        self.dp -= 1;
    }

    fn exec_inc(&mut self) {
        let val = self.data[self.dp];
        self.data[self.dp] = if val == 255 { 0 } else { val + 1 };
    }

    fn exec_dec(&mut self) {
        let val = self.data[self.dp];
        self.data[self.dp] = if val == 0 { 255 } else { val - 1 };
    }

    fn exec_put(&mut self) {
        self.output.push(self.data[self.dp]);
    }

    fn exec_get(&mut self) {
        if let Some(byte) = self.input.pop() {
            self.data[self.dp] = byte;
        }
    }

    fn exec_skip(&mut self) {
        if self.data[self.dp] != 0 {
            return;
        }

        let mut depth = 1;
        while depth > 0 {
            self.ip += 1;
            if self.ip == self.code.len() {
                panic!("Matching Loop not found.");
            }
            match self.code[self.ip] {
                Instruction::Skip => depth += 1,
                Instruction::Loop => depth -= 1,
                _ => {}
            };
        }
    }

    fn exec_loop(&mut self) {
        if self.data[self.dp] == 0 {
            return;
        }

        let mut depth = 1;
        while depth > 0 {
            if self.ip == 0 {
                panic!("Matching Skip not found.");
            }
            self.ip -= 1;
            match self.code[self.ip] {
                Instruction::Skip => depth -= 1,
                Instruction::Loop => depth += 1,
                _ => {}
            };
        }
    }
}
