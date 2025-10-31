


fn add(a: i16, b: i16) -> i16 {
    a + b
}

#[test]
fn test_add() {
    let result = add(2, 3);
    assert_eq!(result, 5);
}

// a >= b
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(56, 98), 14);
    assert_eq!(gcd(101, 10), 1);
}

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let tom = Person {
        name: String::from("Tom"),
        age: 30,
    };

    let someone = tom;
}
