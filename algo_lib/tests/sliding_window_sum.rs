use algo_lib::collections::sliding_window::SlidingWindow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

pub fn solve(input: &mut Input, output: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut x = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();

    let mut res = 0;
    let mut sw = SlidingWindow::new(k, |a, b| a + b);
    for _ in 0..k - 1 {
        sw.push(x);
        x = (a * x + b) % c;
    }

    for _ in k - 1..n {
        sw.push(x);
        res ^= sw.get();
        x = (a * x + b) % c;
    }

    output.print_line(res);
}

fn main() {
    let mut input = algo_lib::io::input::Input::stdin();
    let mut output = algo_lib::io::output::Output::stdout();
    let mut pre_calc = ();
    solve(&mut input, &mut output, 1, &mut pre_calc);
}
