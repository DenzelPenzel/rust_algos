use vector::Vector;

fn main() {
    let mut v = Vector::new();

    v.push(String::from("hello"));
    v.push(String::from("world"));

    assert_eq!(v.len(), 2);
    assert_eq!(v.get(0).map(String::as_str), Some("hello"));

    v.get_mut(0).unwrap().push('!');
    assert_eq!(v.pop(), Some(String::from("world")));
    assert_eq!(v.pop(), Some(String::from("hello!")));
    println!("done")
}
