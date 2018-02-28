#![feature(i128)]

extern crate colored;

mod wu;
use wu::source::*;
use wu::lexer::*;
use wu::parser::Parser;
use wu::visitor::Visitor;

fn run(content: &str) {

  let source = Source::from("main.rs/testing.wu", content.lines().map(|x| x.into()).collect::<Vec<String>>());
  let lexer  = Lexer::default(content.chars().collect(), &source);

  let mut tokens = Vec::new();

  for token_result in lexer {
    if let Ok(token) = token_result {
      tokens.push(token)
    } else {
      return
    }
  }

  let tokens_ref = tokens.iter().map(|x| &*x).collect::<Vec<&Token>>();

  let mut parser  = Parser::new(tokens_ref, &source);

  match parser.parse() {
    Ok(ast) => {
      println!("{:#?}", ast);

      let mut visitor = Visitor::new(&source, &ast);
 
      visitor.visit();
    },

    Err(_) => (),
  }
}

fn main() {
  let test1 = r#"
a: int   = 123
b: float = .123
c: char  = 'b'
d: char  = 'a'
e: str   = r"rawwww"
f: bool  = true

foo := f

a: int:   123
b: float: .123
c: char:  '\n'
d: char:  'a'
e: str:   "raw"
f: bool:  true

bar :: b

hmm: int
  "#;

  let test2 = r#"
(a, b, c) := (1, 2, 3)
(æ, ø): (int, str) = (1000, "world")

(grr): bool: false
(bar): (float): .123

(d, e, f, g) :: (1, "two", 3, 4, "hey")

a
b
c
d
e
f
g
grr
æ
ø
  "#;

  run(&test2);
}