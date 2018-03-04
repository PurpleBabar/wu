#![feature(i128)]

extern crate colored;

mod wu;
use wu::source::*;
use wu::lexer::*;
use wu::parser::{ Parser, ExpressionNode, Expression, };
use wu::visitor::Visitor;
use wu::interpreter::{ Machine, Compiler, };

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

      let block = Expression::new(
        ExpressionNode::Block(ast.clone()),
        TokenElement::Lexeme("<entry point>")
      );

      let mut visitor = Visitor::new(&source, &ast);
 
      match visitor.visit() {
        Ok(_) => {
          let mut machine  = Machine::new();
          let mut compiler = Compiler::new(&mut machine);

          let mut compiled = compiler.compile_entry(&block, "main").unwrap();
          let mut vm       = Machine::new();

          println!("constants:\n{:#?}\n\ncode:\n{:#?}", compiled.constants, compiled.code);

          vm.execute(&mut compiled);

          println!("{:#?}", vm.stack);
        }

        _ => (),
      }
    },

    _ => (),
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

  let test3 = r#"
a: int  = 100
b: bool = false

c := .123

d: str: "communism essentially"
e: str: r"you can't escape \n\n\n"

f :: 'a'

(g, h): (int, bool) = (1000, false)

(one, two, three, four, five) := (1, "two", .3, '4', false)

(foo): float =( (1/2) +  (1))
  "#;

  let test4 = r#"
add :: (a: int, b: int) int -> a + b
  "#;

  run(&test4);
}