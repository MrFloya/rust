

fn id<T: copy send>(t: T) -> T { ret t; }

fn main() {
    let expected = ~100;
    let actual = id::<~int>(expected);
    log(debug, *actual);
    assert (*expected == *actual);
}
