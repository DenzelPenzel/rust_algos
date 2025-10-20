/*
https://cses.fi/problemset/task/3405

You are given an array of n integers.
Your task is to calculate the bitwise or of each window of k elements, from left to right.
In this problem the input data is large and it is created using a generator.

Input
    The first line contains two integers n and k: the number of elements and the size of the window.
    The next line contains four integers x, a, b and c: the input generator parameters.

The input is generated as follows:
    x_1=x
    x_i=(ax_{i-1}+b) mod c for i=2,3,...,n

Output
    Print the xor of all window ors.

Constraints
    1 \le k \le n \le 10^7
    0 \le x, a, b \le 10^9
    1 \le c \le 10^9

Example
    Input:
        8 5
        3 7 1 11

    Output:
        4

Explanation:
    The input array is [3,0,1,8,2,4,7,6].
    The windows are [3,0,1,8,2], [0,1,8,2,4], [1,8,2,4,7] and [8,2,4,7,6],
    and their ors are 11, 15, 15 and 15.
    Thus, the answer is 11 || 15 || 15 || 15 = 4.
*/

use algo_lib::collections::sliding_window::SlidingWindow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, output: &mut Output) {
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
}

fn do_test(raw_input: &str, expected: &str) {
    let mut input = Input::slice(raw_input.as_bytes());
    let mut out_buf = Vec::new();

    {
        let mut output = Output::buf(&mut out_buf);
        solve(&mut input, &mut output);
        output.flush();
    }

    let output_str = String::from_utf8(out_buf).unwrap();
    assert_eq!(output_str.trim(), expected);
}

#[test]
fn run_test() {
    do_test("8 5 3 7 1 11", "4");
}
