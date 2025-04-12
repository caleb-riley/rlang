fn pow(base, power) {
    if power < 1 {
        return 1;
    }

    return base * pow(base, power - 1);
}

fn fact(n) {
    if n < 2 {
        return 1;
    }

    return n * fact(n - 1);
}

fn main() {
    let a = 4;
    let b = 65;
    a = b;
    print(a);
    print(pow(3, 4));

    let obj = { a: 5 } + { b: false };
    print(obj);
    
    let text = input();
    let val = parseint(text);
    print("One more: " + tostring(val + 1));

    define("name", "John");
    print(name);
    
    print(-6);

    let index = 1;

    while index < 10 {
        print(index);
        index = index + 1;
    }
}
