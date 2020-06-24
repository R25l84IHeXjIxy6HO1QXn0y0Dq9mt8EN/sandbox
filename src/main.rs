trait Runnable {
    fn exec(&self, input: &[bool]) -> bool;
}

struct DFA<'a> {
    f: &'a [fn(bool) -> bool],
    x: &'a [usize],
    y: &'a [usize],
    n: &'a [usize]
}

impl Runnable for DFA<'_> {
    fn exec(&self, input: &[bool]) -> bool {
        let mut i = 0;
        let mut n = 0;
        let mut o = true;
        while n < self.f.len() {
            o = (self.f[i])(input[self.x[i]]);
            i = n;
            n = if o {
                self.y[i]
            }
            else {
                self.n[i]
            }
        }
        o
    }
}

fn main() {
    let dfa = DFA {
        f: &[(|x: bool| { !x })],
        x: &[2],
        y: &[1],
        n: &[1]
    };
    println!("{}", dfa.exec(&[true, false, true, false]));
}
