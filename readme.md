Run Rust code through [Rust Playground](https://play.rust-lang.org/).

```sh
$ cat a.rs
fn main() {
    println!("{}", 123);
}
$ cargo build
$ cat a.rs | target/debug/rustplay
123
```

I created this on a whim inspired by [haya14busa/goplay](https://github.com/haya14busa/goplay).
