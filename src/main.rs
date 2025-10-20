use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::collections::sliding_window::SlidingWindow;

fn main() {
    let mut input = Input::stdin();
    let mut output = Output::stdout();

    let n = input.read_size();
    let k = input.read_size();
    let mut x = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();

    let mut res = 0;
    let mut sw = SlidingWindow::new(k, |a, b| a | b);
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
    output.flush();
}
