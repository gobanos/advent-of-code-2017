#[macro_use] extern crate nom;

use nom::anychar;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Snd(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Value {
    Register(char),
    Literal(i32),
}

named!(register<char>, verify!(anychar, |c| c >= 'a' && c <= 'z'));

named!(snd<Instruction>, do_parse!(
    tag!("snd ")    >>
    reg: register   >>

    (Instruction::Snd(reg))
));

named!(rcv<Instruction>, do_parse!(
    tag!("rcv ")    >>
    reg: register   >>

    (Instruction::Rcv(reg))
));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
