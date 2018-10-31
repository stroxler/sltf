use std::collections::HashMap;

extern crate either;
use either::{Either, Left, Right};

#[cfg(test)] #[macro_use]
extern crate pretty_assertions;

extern crate sltf_parse;
use sltf_parse::Ast;
use sltf_parse::Atom;
use sltf_parse::Prim;

mod builtin_words;

// Note: I failed to figure out rust closures and gave up.
//
// The fn type is not a closure, but rather explicitly a static
// function, which means it can be used as a stack value because it's
// size (a pointer to the start instruction) is known at compile time.
// This is not true of closures, which need to be boxed.
//
// I also failed to make the non-closure version work, because
// in order to find the value of the word in WordMap, I had to look it
// up. In the process, I have a borrow out on `vm`, which means I cannot
// do anything with it.
//
// This isn't actually a problem because I'm in tail position, meaning that
// 
type Stack = Vec<Prim>;
type Prog = Vec<Ast>;
type WordExec = fn(&mut Stack);
type WordBody = Vec<Atom>;
type WordMap = HashMap<String, Either<WordExec, WordBody>>;


pub struct Vm {
    pub stack: Stack,
    prog: Prog,
    word_map: WordMap,
}



impl Vm {

    pub fn new(prog_in: Vec<Ast>) -> Self {
        let mut prog = prog_in;
        prog.reverse();
        Vm {
            stack: Vec::new(),
            prog,
            word_map: builtin_words::initial_word_map(),
        }
    }

    pub fn new_input(&mut self, prog_in: Vec<Ast>) {
        let mut prog = prog_in;
        prog.reverse();
        prog.append(&mut self.prog);
        self.prog = prog;
    }

    pub fn finished(&self) -> bool {
        self.prog.len() == 0
    }

    pub fn execute(&mut self) {
        let item = self.prog.pop().expect("Called execute on vm with empty prog");
        match item {
            Ast::Simple(atom) => {
                self.execute_atom(atom)
            }
            Ast::WordDef(name, body) => {
                let old = self.word_map.insert(
                    name.clone(),
                    Either::Right(body),
                );
                match old {
                    Some(_) => println!("Redefined word {:?}", name),
                    None => println!("Defined new word {:?}", name),
                };
            }
        }
    }


    fn execute_atom(&mut self, atom: Atom) {
        match atom {
            Atom::Lit(prim) => {
                self.stack.push(prim);
            },
            Atom::Symbol(word) => {
                match self.word_map.get(&word) {
                    Some(Left(func)) => {
                        func(&mut self.stack)
                    },
                    Some(Right(body)) => {
                        expand_word_def(&mut self.prog, &body);
                    },
                    None => {
                        panic!("Could not find definition of word {:?}", &word);
                    },
                }
            },
        };
    }

}

fn expand_word_def(prog: &mut Prog, body: &Vec<Atom>) {
    for atom in body.iter().rev() {
        let copied = atom.clone();
        prog.push(Ast::Simple(copied));
    };
}

#[test]
fn vm_new_input_test() {
    let mut vm = Vm::new(vec![
        Ast::Simple(Atom::Lit(Prim::Int(2))),
        Ast::Simple(Atom::Lit(Prim::Int(3))),
    ]);
    vm.new_input(vec![
        Ast::Simple(Atom::Lit(Prim::Int(4))),
        Ast::Simple(Atom::Lit(Prim::Int(5))),
    ]);
    for _ in 0..4 {
        vm.execute();
    }
    assert_eq!(vm.stack, vec![
        Prim::Int(2),
        Prim::Int(3),
        Prim::Int(4),
        Prim::Int(5),
    ])
}



#[cfg(test)]
mod vm_execute_tests {
    use super::*;

    fn _tcase(prog: Vec<Ast>,
              n_executions: Option<u64>,
              expected_stack: Vec<Prim>) {
        let mut vm = Vm::new(prog);
        match n_executions {
            None => {
                while vm.prog.len() > 0 {
                    vm.execute()
                }
            },
            Some(n) => {
                for _ in 0..n {
                    vm.execute()
                }
            },
        };
        println!("stack: {:?}", &vm.stack);
        assert_eq!(vm.stack, expected_stack);
    }

    fn tcase_run_all(prog: Vec<Ast>,
                    expected_stack: Vec<Prim>) {
        _tcase(prog, None, expected_stack);
    }

    fn tcase_run_n(prog: Vec<Ast>,
                   expected_stack: Vec<Prim>,
                   n: u64) {
        _tcase(prog, Some(n), expected_stack);
    }

    fn simple_lit(prim: Prim) -> Ast {
        Ast::Simple(Atom::Lit(prim))
    }

    fn simple_sym(name: &str) -> Ast {
        Ast::Simple(Atom::Symbol(name.to_string()))
    }

    #[test]
    fn test_push_integers() {
        tcase_run_all(
            vec![
                simple_lit(Prim::Int(2)),
                simple_lit(Prim::Int(3)),
            ],
            vec![
                Prim::Int(2),
                Prim::Int(3),
            ],
        );
    }

    #[test]
    fn test_add() {
        tcase_run_all(
            vec![
                simple_lit(Prim::Int(2)),
                simple_lit(Prim::Int(3)),
                simple_sym("+"),
            ],
            vec![
                Prim::Int(5),
            ],
        );
    }

    #[test]
    fn test_arith() {
        tcase_run_all(
            vec![
                simple_lit(Prim::Int(2)),
                simple_lit(Prim::Int(3)),
                simple_lit(Prim::Int(4)),
                simple_sym("+"),
                simple_sym("*"),
            ],
            vec![
                Prim::Int(14),
            ],
        );
    }

    #[test]
    fn test_stack_dup_fns() {
        tcase_run_all(
            vec![
                simple_lit(Prim::Int(2)),
                simple_lit(Prim::Int(3)),
                simple_sym("dup2"),
                simple_sym("dup"),
            ],
            vec![
                Prim::Int(2),
                Prim::Int(3),
                Prim::Int(2),
                Prim::Int(3),
                Prim::Int(3),
            ],
        );
    }

    #[test]
    fn test_swap() {
        tcase_run_all(
            vec![
                simple_lit(Prim::Int(2)),
                simple_lit(Prim::Int(3)),
                simple_sym("swap"),
            ],
            vec![
                Prim::Int(3),
                Prim::Int(2),
            ],
        );
    }
}

