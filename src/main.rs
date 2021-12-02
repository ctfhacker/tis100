use std::fmt::Display;
use std::ops::Neg;

/// Communication channel between nodes in the CPU
#[derive(Debug)]
enum Port {
    /// Left communication channel from a node
    Left,

    /// Right communication channel from a node
    Right,

    /// Up communication channel from a node
    Up,

    /// Down communication channel from a node
    Down,

    /// First available port of Left/Right/Up/Down
    Any,

    /// The last read or written used port by [`Port::Any`]
    Last
}

/// Operational unit used by the CPU nodes
#[derive(Debug)]
enum Value {
    /// Raw literal number
    Number(i16),

    /// Communication register to read/write
    Port(Port)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => {
                write!(f, "{}", num)
            }
            Value::Port(port) => {
                write!(f, "{:?}", port)
            }
        }
    }
}

/// An instruction to execute in a node in the CPU
#[derive(Debug)]
enum Opcode {
    /// Add the [`Value`] to the `ACC` register and store the result back into `ACC`
    Add(Value),

    /// Subtract the [`Value`] from the `ACC` register and store the result back into the
    /// `ACC` 
    Sub(Value),

    /// The values of `ACC` and `BAK` are exchanged
    Swap,

    /// The values of `ACC` is written to `BAK`
    Save,

    /// The value of `ACC` is arithmetically negated. Zero remains the same.
    Negate
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Add(val) => write!(f, "ADD {}", val),
            Opcode::Sub(val) => write!(f, "SUB {}", val),
            Opcode::Swap     => write!(f, "SWP"),
            Opcode::Save     => write!(f, "SAV"),
            Opcode::Negate   => write!(f, "NEG"),
        }
    }
}


/// An individual node of execution in the CPU
#[derive(Debug, Default)]
struct Node {
    /// The `ACC` register
    acc: i16,

    /// The `BAK` register
    bak: i16,

    /// The opcodes executing in this [`Node`]
    opcodes: Vec<Opcode>,

    /// The current instruction being executed
    pc: usize
}

impl Node {
    pub fn step(&mut self) {
        let curr_op = &self.opcodes[self.pc];

        match curr_op {
            Opcode::Add(val) | Opcode::Sub(val) => {
                // Get the underlying value
                let num = match val {
                    Value::Number(num) => num,
                    Value::Port(_port)  => unimplemented!()
                };

                // Perform the operation
                match curr_op {
                    Opcode::Add(_) => self.acc += num,
                    Opcode::Sub(_) => self.acc -= num,
                    _ => unreachable!()
                }

                // Clamp value to within bounds
                self.acc = self.acc.clamp(-999, 999);
            }
            Opcode::Swap => std::mem::swap(&mut self.acc, &mut self.bak),
            Opcode::Save => self.bak = self.acc,
            Opcode::Negate => self.acc = self.acc.neg()
        }

        println!();

        // Go to the next instruction
        self.pc += 1;

        // Loop the program once at the end
        if self.pc >= self.opcodes.len() {
            self.pc = 0;
        }
    }
}

/// Collection of executing [`Nodes`]
#[derive(Debug, Default)]
struct Cpu {
    /// Individual node units for the CPU
    nodes: [Node; 4]
}

impl Cpu {
    pub fn step(&mut self) {
        for node in self.nodes.iter_mut() {
            node.step();
        }
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Width of each node to print
        let node_width   = 22;

        // Padding between nodes
        let node_padding = " ".repeat(6);

        // Number of nodes in each column
        let columns = 2;

        for row in 0..2 {
            // Print the header line
            write!(f, "+{0:}+{1:}+{0:}+\n", 
                "-".repeat(node_width), 
                node_padding)?;

            for instr in 0..6 {
                // Before printing any instructions, print the register values
                if instr == 0 {
                    for col in 0..columns {
                        // Print the left mode barrier
                        write!(f, "|")?;

                        // Get the node index for the current node
                        let node_index = row * columns + col;

                        write!(f, 
                            "{:width$}",
                            format!(" ACC: {:4} BAK: {:4}", self.nodes[node_index].acc,
                                self.nodes[node_index].bak),
                            width = node_width)?;

                        write!(f, "|{}", node_padding)?;
                    }

                    // Print new line
                    write!(f, "\n")?;

                    // Print the barrier between registers and opcodes
                    write!(f, "+{0:}+{1:}+{0:}+\n", 
                        "-".repeat(node_width), 
                        node_padding)?;
                }

                for col in 0..columns {
                    // Print the left mode barrier
                    write!(f, "|")?;

                    // Get the node index for the current node
                    let node_index = row * columns + col;

                    if self.nodes[node_index].pc == instr {
                        write!(f, "> ")?;
                    } else {
                        write!(f, "  ")?;
                    }

                    match self.nodes[node_index].opcodes.get(instr) {
                        Some(opcode) => {
                            let _ = write!(f, "{:width$}", 
                                format!("{}", opcode), 
                                width = node_width - 2);
                        }
                        None => {
                            let _ = write!(f, "{:width$}", " ", width = node_width - 2);
                        }
                    }

                    write!(f, "|{}", node_padding)?;
                }

                write!(f, "\n")?;
            }

            // Print the header line
            write!(f, "+{0:}+{1:}+{0:}+\n", 
                "-".repeat(node_width), 
                node_padding)?;

            // Spacing between nodes
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    // Create the default CPU
    let mut cpu  = Cpu::default();

    // Fakes test nodes
    cpu.nodes[0].opcodes.push(Opcode::Add(Value::Number(1)));
    cpu.nodes[0].opcodes.push(Opcode::Save);
    cpu.nodes[0].opcodes.push(Opcode::Add(Value::Number(1)));
    cpu.nodes[0].opcodes.push(Opcode::Swap);
    cpu.nodes[0].opcodes.push(Opcode::Negate);

    cpu.nodes[1].opcodes.push(Opcode::Add(Value::Number(2)));
    cpu.nodes[1].opcodes.push(Opcode::Sub(Value::Number(400)));

    cpu.nodes[2].opcodes.push(Opcode::Add(Value::Number(-400)));

    cpu.nodes[3].opcodes.push(Opcode::Add(Value::Number(1)));
    cpu.nodes[3].opcodes.push(Opcode::Sub(Value::Number(2)));
    cpu.nodes[3].opcodes.push(Opcode::Sub(Value::Number(4)));
    cpu.nodes[3].opcodes.push(Opcode::Sub(Value::Number(5)));

    // Init destination string for read_line 
    let mut input = String::new();

    // "Debugger" loop.. Waits for enter to step to the next iteration
    loop {
        println!("{}", cpu);
        std::io::stdin().read_line(&mut input)?;
        cpu.step();

        if input.to_ascii_lowercase().contains('q') {
            break;
        }
    }

    Ok(())
}
