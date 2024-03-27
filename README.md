# rust_practice
my practice for learning rust

## Day01 Notes
### Install rust & visual studio code's extensions
#### Install Rust:
- __[Rust-lang](https://www.rust-lang.org/tools/install)__

#### Visual studio code extensions
- __[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)__
- __[Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)__
- __[Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)__

### About rust common commands
1. rustup
1. rustc
### About rust's package management tool
1. cargo


## Day02 Notes
### Ch02_01 Variables
#### Declare immutable varianle
- Let x: i32 = 5; or let x = 5; x is i32
#### Declare mutable varianle
- Let mut x: i32 = 5; or let mut x = 5; x is i32
#### Shadowing
##### Shadowing
    fn variables_shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value of x is: {}", x);
        // The value of x is: 12
    }
##### Non-Shadowing
    fn variables_shadowing() {
        let x = 5;
        let y = x + 1;
        let z = y * 2;
        println!("The value of z is: {}", z);
        // The value of z is: 12
    }
### shadowing vs mutable
`1. The shadowing can change datatype, but mutable can't.`
`2. The shadowing is alway immutable.`
#### Naming rule for variable
1. Snake Case: Let _nice_count : i32 = 0;
1. Pascal Case(For enumerate and structure)
#### Casting a value to a different type
    Let __nice_count_one : i32 = 0;
    Let _nice_count_two = __nice_count_one as i32;
#### Print out variable
`println!("Value is: {}", __nice_count_one);`
`println!("Value is: {__nice_count_one}");`

