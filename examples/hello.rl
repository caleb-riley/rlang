fn power(base, power) {
    if power < 1 {
        return 1;
    }

    return base * power(base, power - 1);
}

fn factorial(n) {
    if n < 2 {
        return 1;
    }

    return n * factorial(n - 1);
}

fn test_assignment_and_functions() {
    print("Testing assignment and function calls");

    let a = 4;
    let b = 65;
    a = b;

    print(a);
    print(power(3, 4));
}

fn test_objects() {
    print("Testing object creation and addition");

    let obj = { a: 5 } + { b: false };

    print(obj);
}

fn test_input_and_parsing() {
    print("Testing prompting and string parsing");

    let text = prompt("Please enter a number: ");
    let val = parseint(text);

    print(text + " + 1 = " + tostring(val + 1));
}

fn test_lists() {
    print("Testing list appending and indexing");

    let index = 0;
    let squares = [];

    while index < 10 {
        append(squares, index * index);
        index = index + 1;
    }

    let index = 0;

    while index < 10 {
        let value = get(squares, index);
        print(tostring(index) + " squared is " + tostring(value));
        index = index + 1;
    }
}

fn main() {
    test_assignment_and_functions();
    test_objects();
    test_input_and_parsing();
    test_lists();
}
