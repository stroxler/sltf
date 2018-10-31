# SLTF: a toy interpreter for a postfix extensible calculator

I decided to learn rust, and needed a project to get going. Since I've
been looking a lot at programming languages lately, a tiny interpreter
seemed to make sense.

Since Forth is considered among the simplest languages, I decided to
make a forth-ish interpreter. Mine is very simple, though, and cannot
easily be extended to a "real" interpreter because the VM mechanics
are too limited. Nontheless I did write a parser and an evaluator for
a postfix notation stack machine, included a few of the forth builtin
words, and provided the ability to define new words.

Here's a sample session
```
> cargo run
 ok
1 2
 ok
.
 Int(1) Int(2) ok
+
 ok
: square dup * ;
Defined new word "square"
 ok
square
 ok
.
 Int(9) ok
 ```

## Implementation notes - this is nowhere near a "real" interpreter

This "forth-ish postfix stack machine" is very much not a full forth.

My vm is different from more realistic forth vms in several ways, because
at the time of writing I understood very little about the forth language:
 - I have a much smaller set of builtins implemented; at the moment just
   `drop`, `dup`, `dup2`, `swap`, `+`, `-`, and `.`.
 - There is no return stack.
 - Constructs like `if` and `loop` aren't really possible with the
   limited mechanics I currently have; I need to read more about how they
   are implemented in Forth.
 - My handling of strings is very different from real forth; they have
   macro-type tools for printing strings by generating low-level code,
   whereas my interpreter supports first-class strings (although there
   aren't any builtin words that act on them).

Another thing that I *think* is unusual about my implementation: the
program itself is stored as a stack, where we pop off of the left-hand
side. I actually rather like this approach to a postfix program, although
my current implementation is too limited to get the full benefit. At
the moment I allow word definitions to modify the program stack, but
nothing else.

I *think* that most of the special constructs should be implementable by
modifying the "program stack", e.g. and `if` looks at the top of the data stack
and then puts one of the branches of code on the program stack. A `loop`
has to have a variable index; if it does, then it can put the next copy
of `loop` on the stack, then the body, and continue execution.

## Done, at least for now

I'm finished working on this project for now; it's very clear to me that I want
to read more rust programs before doing another rust project, because the
design patterns around handling the borrow checker aren't at all clear yet.
I have other projects to work on in the meantime.

I'm hoping to come back to this eventually, and:
  - provide a mechanism to allow words to modify both the stack and
    the "program stack". I might have a special notation for such words,
    e.g. using :: instead of : to start them.
  - alternatively, provide a mechanism to work with "lists" of unevaluated
    words that can live in the data stack (see the STCK programming language)
  - create a return stack
  - handle strings differently, either:
    - get rid of strings and do what forth does
    - leave strings in, and add tools to work with them

# Some general notes on lexing

There are a bunch of ways to tokenize (parser-combinators, lexer generators,
raw tokenization) but a pretty common one is to use regular expressions
(most lexer generators are regex-based, and parser cominator approaches
basically wind up implementing a regexp DSL anyway).

Since I didn't want to learn a lexer generator or parser combinator
library at this time, and Rust doesn't have pattern matching for strings,
I decided to use a pretty standard regexp approach:
  - I define a regexp for each type of token. For the most part these
    are very simple expressions (e.g. a number)
  - mostly sharing the same code, I define a top-level regexp for any
    type of token, using a big "or" inside a match, and allowing
    whitespace around each side

The tricky parts of this were:
  - figuring out how to extract the match. Except for strings, I was
    able to just use a big match around the entire token "or" expression
    to get the raw token, and then process it.
  - figuring out the semantics of the regexp library. The match function
    is happy to match somewhere in the middle of the text unless you
    anchor using `^` and `$`, so I had to add these around each of the
    token types.
  - Dealing with string complications: what should the match expression
    be, and also how do we post process? My solution was that
    - The match expression is `r#""((?:\\.|[^\\"]))""#`. What does this
      mean?
      - We have double-quotes surrounding a match group
      - In the match group, we have arbitrary escaped characters *or*
        any character other than a literal `\` or `"`. The only reason
        we cannot allow `\` is because we don't want to consume single
        backslashes and then miss an escape later
      - Once we get the raw string, we need to convert all the escape sequences into
        actual characters. There's probably a libraryto do this, but I just
        did a bunch of repaces by hand

I actually used the rust implementation of `mal` to help me work out the
correct regex usage for tokenization, so a lot of what I learned is
probably generalizable to other languages.

# Next steps in rust

My plan in the short run with rust is to focus on reading (and possibly
re-implementing) and going through screencasts.

Reading rust books: I think at this point I know enough that it's easier
to learn the basics by projects / examples rather than from books, but there
are several project-based and advanced-topic things out there:
 - The rustonomicon
 - The minibook on linked lists in Rust
 - The blog/book about rust operating systems

Reading rust interpreters:
 - [the mal rust implementation](https://github.com/kanaka/mal/tree/master/rust)
 - [the RustPython interpreter](https://github.com/RustPython/RustPython)
 - [the rust-prolog interpreter](https://github.com/dagit/rust-prolog)
 - [possibly shen-rust](https://github.com/deech/shen-rust)
   I'm not sure this actually works, but if so it's probably a good way to
   learn about closures in rust.

Other rust reading:
 - [llamadb, a toy SQL database](https://github.com/nukep/llamadb)
   - There are a few forks that are actually better
   - [has minor fixes in master](https://github.com/imor/llamadb)
   - [has minor fixes in one branch, small extensions in another](https://github.com/acmiyaguchi/llamadb/tree/SqlEngine)
   - [has significant progress](https://github.com/fluencelabs/llamadb)
     ...Keep an eye on this project, it seems to be making active progress.
     I'm not sure if they are just updating at a toy project, or if they
     hope ot make use of it; fluencelabs appears to have actually used
     llamadb in a workshop on their product(!!!)


Forth reading:
 - [easy forth](https://github.com/skilldrick/easyforth/tree/gh-pages/javascripts)
 - forth books: Starting Forth, Thinking Forth, Program Forth
 - jonesforth and sixtyfourth
 - gforth and pforth
