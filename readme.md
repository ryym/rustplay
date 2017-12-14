# rustplay

Run Rust code through [Rust Playground](https://play.rust-lang.org/).

```sh
$ cat a.rs
fn main() {
    println!("{}", 123);
}
$ rustplay -r a.rs
123
$ rustplay -o a.rs
# => Open Playground website
```

```sh
$ rustplay -h
Usage: rustplay [options] FILE
NOTE: -r or -o is required

Options:
    -h, --help          print this help message
    -r, --run           compile and run given code using Rust Playground
    -o, --open          open Rust Playground with given code
    -c, --channel stable|beta|nightly
                        chose release channel which compiles code
    -m, --mode debug|release
                        chose compilation mode
```

I created this on a whim inspired by [haya14busa/goplay](https://github.com/haya14busa/goplay).
