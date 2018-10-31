use std::collections::HashMap;

use either::{Either, Left, Right};

use super::Stack;
use super::WordMap;
use super::Prim;


pub fn initial_word_map() -> WordMap {
    let mut word_map: WordMap = HashMap::new();
    word_map.insert("drop".to_string(), Left(bi_drop));
    word_map.insert("dup".to_string(), Left(bi_dup));
    word_map.insert("dup2".to_string(), Left(bi_dup2));
    word_map.insert("swap".to_string(), Left(bi_swap));
    word_map.insert("+".to_string(), Left(bi_add));
    word_map.insert("*".to_string(), Left(bi_mult));
    word_map.insert(".".to_string(), Left(bi_show_stack));
    word_map
}

fn bi_drop(stack: &mut Stack) {
    stack.pop();
}

fn bi_show_stack(stack: &mut Stack) {
    for prim in stack {
        print!(" {:?}", prim);
    }
}

fn bi_dup(stack: &mut Stack) {
    // Note: the borrow checker is smart enough to know that if there's
    // a clone() call at the end of a line, we have not created a reference
    // to `stack`. If you try to put the clones inside the push statements,
    // it will complain because stack cannot be modified when i0 refers
    // to data in the stack.
    let i0 = stack.last().expect("Called dup on empty stack").clone();
    stack.push(i0);
}

fn bi_dup2(stack: &mut Stack) {
    if (stack.len() < 2) {
        panic!("Called dup on stack with fewer than two elements")
    }
    let i0 = stack[stack.len() - 1].clone();
    let i1 = stack[stack.len() - 2].clone();
    stack.push(i1);
    stack.push(i0);
}

fn bi_swap(stack: &mut Stack) {
    // Note that no clones are needed here because, unlike indexing or
    // using last(), popping from the stack does *not* create a borrow of
    // something in the stack
    let i0 = stack.pop().expect("Called swap on empty stack");
    let i1 = stack.pop().expect("Called swap on stack with one element");
    stack.push(i0);
    stack.push(i1);
}


fn bi_add(stack: &mut Stack) {
    let i0 = stack.pop().expect("Called + on empty stack");
    let i1 = stack.pop().expect("Called + on stack with one element");
    let out = match (i0, i1) {
        (Prim::Int(n0), Prim::Int(n1)) =>
          Prim::Int(n0 + n1),
        (bad0, bad1) =>
          panic!("Called + without int args at top of stack: {0:?}, {1:?}", bad1, bad0),
    };
    stack.push(out);
}


fn bi_mult(stack: &mut Stack) {
    let i0 = stack.pop().expect("Called + on empty stack");
    let i1 = stack.pop().expect("Called + on stack with one element");
    let out = match (i0, i1) {
        (Prim::Int(n0), Prim::Int(n1)) =>
          Prim::Int(n0 * n1),
        (bad0, bad1) =>
          panic!("Called + without int args at top of stack: {0:?}, {1:?}", bad1, bad0),
    };
    stack.push(out);
}