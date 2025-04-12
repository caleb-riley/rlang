# ®️ R-Lang

R-Lang is a scripting language I wrote as a personal project using the [Rust](https://www.rust-lang.org/) programming language. It offers basic features such as functions, variables, and objects.

## Installation

To install the R-Lang interpreter, clone the repository to your machine and execute the parser/interpreter using `cargo`.

```
# Clone the repository to your machine
$ git clone https://github.com/caleb-riley/rlang.git

# Run the program using cargo
$ cargo r --release PATH_TO_FILE args...
```

## Syntax

### The Main Function

The main program is defined as such:

```
fn main() {
    print("Hello, world!");
}
```

R-Lang also supports command-line arguments, whose count must match the number of parameters of the main function.

```
fn main(arg) {
    print("Hello, " + arg + "!");
}
```

When running this example, you would execute the following command with a single argument:

```
$ cargo r --release ./main.rl John
Hello, John!
```

### Objects

Objects are another feature of R-Lang. They are basically just hash maps from field names to values, offering dynamic storage of various data types. To create an object, we use curly brace syntax.

```
fn main() {
    let obj = { flag: true };
    print(obj);
}
```

Running it would give the following output:

```
$ cargo r --release ./object.rl
{ flag: true }
```

Objects can be combined using the `+` operator, copying the values of the object into a new object.

```
fn main() {
    let obj = { flag: true } + { count: 2 };
    print(obj);
}
```

Running it would give the following output:

```
$ cargo r --release ./object.rl
{ flag: true, count: 2 }
```

## Built-in Functions

R-Lang offers a collection of built-in functions to make your life easier and provide additional functionality that can't be defined by the developer. They do not require any imports and can be used from anywhere in your program.

| Name | Arguments | Return | Purpose |
|:----:|:---------:|:------:|:--------|
| `print` | `val`: `any` | `null` | Prints the string representation of a value to the console |
| `input` | none | `string` | Accepts text input from the console and returns it as a string |
| `parseint` | `num`: `string` | `number` | Parses a string into an integer and returns its value |
| `tostring` | `val`: `any` | `string` | Just like `print`, but returns the string instead of printing it |
| `define` | `name`: `string` `val`: `any` | `null` | Declares a variable with the given name in the current scope, assigning it the provided value |

## License

This program is free to use and does not require a license. This was made for educational and learning purposes, and I encourage anyone to expand upon it and add new features as they wish.
