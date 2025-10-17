use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::iter;

fn main() {
    let mut input = Input::stdin();
    let mut output = Output::stdout();

    let n = input.read_size();
    let k = input.read_size();
    let x = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();

    let res = iter::successors(Some(x), |xx| Some((a * xx + b) % c))
        .take(n)
        .enumerate()
        .filter(|(i, _)| (i + 1).min(n - i).min(k) % 2 == 1)
        .fold(0, |xor, (_, v)| xor ^ v);

    output.print_line(res);
    output.flush();
}
