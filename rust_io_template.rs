//Input template in rust for competitve programming
use std::io;
use std::io::{BufRead, Write};

fn solve<R: BufRead, W: Write>(mut input: R, mut output: W) {
    let mut buf = String::new();

    //Input first line with single character
    input.read_line(&mut buf).unwrap();
    let _n: usize = buf.trim_end()
                       .parse()
                       .unwrap();

    buf.clear(); // clear buffer before each input

    //Input second line with multiple characters
    input.read_line(&mut buf).unwrap();
    let v: Vec<usize> = buf.split_whitespace()
                           .map(|s| s.parse().unwrap())
                           .collect();

    writeln!(output, "{} {:?}", _n, v).unwrap();
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod test {
    fn test(input: &str, output: &str) {
        let mut v: Vec<u8> = Vec::new();
        ::solve(input.as_bytes(), &mut v);
        assert_eq!(String::from_utf8(v).unwrap(), output);

    }

    #[test]
    fn case1(){
        test (
            //Enter input inlcuding new line
            //Sample 
            "3
            1 2 3 4 5 6",

            //Enter output including new line
            //Sample
            "3 [1, 2, 3, 4, 5, 6]\n"
        );
    }
}