use std::io;

extern crate sltf_parse;
extern crate sltf_vm;

use sltf_parse::{Prim,Parser};
use sltf_vm::{Vm};


fn main() {
    use io::BufRead;
    let mut runner = Runner::new("");
    let stdin = io::stdin();
    println!(" ok");
    for line in stdin.lock().lines() {
        runner.parse_input(line.unwrap().as_ref());
        runner.run();
        println!(" ok");
    }
}


struct Runner {
    parser: Parser,
    vm: Vm,
}

impl Runner {

    fn new(initial_input: &str) -> Self {
        let parser = Parser::new();
        let prog = parser.parse(initial_input);
        let vm = Vm::new(prog);
        Runner {
            parser,
            vm,
        }
    }

    fn parse_input(&mut self, input: &str) {
        let prog = self.parser.parse(input);
        self.vm.new_input(prog);
    }

    fn execute(&mut self) {
        self.vm.execute()
    }

    fn run(&mut self) {
        while !self.vm.finished() {
            self.execute()
        }
    }
}
    

mod runner_tests {
    use super::*;

    fn tcase_from_init(init: &str,
                       expected_stack: Vec<Prim>) {
        let mut runner = Runner::new(init);
        runner.run();
        assert_eq!(runner.vm.stack,  expected_stack);
    }

    #[test]
    fn test_runner_simple() {
        tcase_from_init(
            "2 3 4 + 5 *",
            vec![
                Prim::Int(2),
                Prim::Int(35),
            ]
        );
    }

    #[test]
    fn test_runner_complex() {
        tcase_from_init(
            ": square dup * ; 2 dup square + 35 swap",
            vec![
                Prim::Int(35),
                Prim::Int(6),
            ]
        );
    }

}