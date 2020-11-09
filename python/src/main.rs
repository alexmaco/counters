use rustpython_parser::token::Tok;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::io::{self, BufRead};

use rustpython_parser::lexer::Lexer;

fn main() {
    let stdin = io::stdin();
    let mut bag = HashMap::<OTok, usize>::new();
    for line in stdin.lock().lines() {
        let f = line.unwrap();
        eprintln!("file: {}", f);
        let text = fs::read_to_string(f).unwrap();
        let lex = Lexer::new(text.chars());
        for res in lex {
            let (_, tok, _) = res.unwrap();
            *bag.entry(OTok(tok)).or_insert(0) += 1;
        }
    }
    let mut l: Vec<_> = bag
        .iter()
        .map(|(tok, num)| (format!("{}", tok.0), *num))
        .collect();
    l.sort_by(|x, y| y.1.cmp(&x.1));
    let j = serde_json::to_string_pretty(&l);
    println!("{}", j.unwrap());
    eprintln!("total: {}", l.iter().map(|(_, n)| *n).sum::<usize>());
}

#[derive(Debug, PartialEq)]
struct OTok(Tok);

impl Eq for OTok {}
impl Hash for OTok {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        use ordered_float::OrderedFloat as OF;
        let disc = std::mem::discriminant(&self.0);
        disc.hash(state);
        use Tok::*;
        match &self.0 {
            Name { name } => name.hash(state),
            Int { value } => value.hash(state),
            Float { value } => OF(*value).hash(state),
            Complex { real, imag } => {
                OF(*real).hash(state);
                OF(*imag).hash(state);
            }
            String { value, is_fstring } => {
                value.hash(state);
                is_fstring.hash(state);
            }
            Bytes { value } => value.hash(state),
            Newline => {}
            Indent => {}
            Dedent => {}
            StartProgram => {}
            StartStatement => {}
            StartExpression => {}
            EndOfFile => {}
            Lpar => {}
            Rpar => {}
            Lsqb => {}
            Rsqb => {}
            Colon => {}
            Comma => {}
            Semi => {}
            Plus => {}
            Minus => {}
            Star => {}
            Slash => {}
            Vbar => {}  // '|'
            Amper => {} // '&'
            Less => {}
            Greater => {}
            Equal => {}
            Dot => {}
            Percent => {}
            Lbrace => {}
            Rbrace => {}
            EqEqual => {}
            NotEqual => {}
            LessEqual => {}
            GreaterEqual => {}
            Tilde => {}
            CircumFlex => {}
            LeftShift => {}
            RightShift => {}
            DoubleStar => {}
            DoubleStarEqual => {} // '**='
            PlusEqual => {}
            MinusEqual => {}
            StarEqual => {}
            SlashEqual => {}
            PercentEqual => {}
            AmperEqual => {} // '&='
            VbarEqual => {}
            CircumflexEqual => {} // '^='
            LeftShiftEqual => {}
            RightShiftEqual => {}
            DoubleSlash => {} // '//'
            DoubleSlashEqual => {}
            ColonEqual => {}
            At => {}
            AtEqual => {}
            Rarrow => {}
            Ellipsis => {}

            // Keywords (alphabetically):
            False => {}
            None => {}
            True => {}

            And => {}
            As => {}
            Assert => {}
            Async => {}
            Await => {}
            Break => {}
            Class => {}
            Continue => {}
            Def => {}
            Del => {}
            Elif => {}
            Else => {}
            Except => {}
            Finally => {}
            For => {}
            From => {}
            Global => {}
            If => {}
            Import => {}
            In => {}
            Is => {}
            Lambda => {}
            Nonlocal => {}
            Not => {}
            Or => {}
            Pass => {}
            Raise => {}
            Return => {}
            Try => {}
            While => {}
            With => {}
            Yield => {}
        }
    }
}
