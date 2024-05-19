use std::collections::HashMap;

pub struct BFExtended {
    memory: HashMap<usize,u8>,
    memory_pointer: usize,
    pointer: usize,
    jump_stack: Vec<usize>
}

impl BFExtended {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
            memory_pointer: 0,
            pointer: 0,
            jump_stack: Vec::new()
        }
    }
    
    fn access_mem(&self,address: usize) -> u8 {
        match self.memory.get(&address) {
            Some(v) => *v,
            None => 0u8
        }
    }

    fn write_mem(&mut self,address: usize,value: u8) {
        self.memory.insert(address,value);
    }

    pub fn run(&mut self, code: &String){
        let code = code.as_bytes();
        
        loop {
            if code.len() <= self.pointer {
                break;
            }        
     
            let mut increment: bool = true;
            
            match code[self.pointer] {
                b'+' => {
                    let old = self.access_mem(self.memory_pointer);
                    self.write_mem(self.memory_pointer,if old == 255 { 0u8 } else { old + 1 }); 
                },
                b'-' => {
                    let old = self.access_mem(self.memory_pointer);
                    self.write_mem(self.memory_pointer,if old == 0 { 255u8 } else { old - 1 }); 
                },
                b'>' => {
                    self.memory_pointer += 1
                },
                b'<' => self.memory_pointer -= 1,
                b'[' => {
                   self.jump_stack.push(self.pointer+1); 
                },
                b']' => {
                   let last = self.jump_stack.last().expect("failed to get last element.");
                   if self.access_mem(self.memory_pointer) != 0 {
                     self.pointer = *last;
                     increment = false;
                   } else {
                     self.jump_stack.pop();
                   }
                },
                b'.' => {
                    print!("{}",char::from(self.access_mem(self.memory_pointer)));
                },
                b'?' => {
                    let cell_value = self.access_mem(self.memory_pointer);
        
                    println!("Code Pointer Location: {}",self.pointer);
                    println!("Memory Pointer Location: {}",self.memory_pointer);
                    println!("Current Cell Value: {}",cell_value);
                    println!("Current Cell Value as Character: ( {} )",char::from(cell_value));
                    println!("Jump Stack: {:?}",self.jump_stack);
                    println!("========");
                },
                b'!' => {
                    for (addr, value) in &self.memory {
                        println!("Address: {} Value: {}",addr,value);
                    }
                    println!("========");
                },
                _ => {}
            };

            if increment { self.pointer += 1 }
        } 
    }
}
