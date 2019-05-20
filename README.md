Crate for formatting `.toml` files using the [`toml`](https://crates.io/crates/toml) crate.

```
$ cat examples/test.toml

[foo]
    data1 = { x = 1,   y = 2 }
data2 = {         x = 1, y = 2 }

      [bar]
data1 = { x = 2,     y = 1 }
  data2 = { x = 2,      y = 1 }


$ cat examples/test.toml | cargo run

[foo.data1]
x = 1
y = 2

[foo.data2]
x = 1
y = 2
[bar.data1]
x = 2
y = 1

[bar.data2]
x = 2
y = 1
```

# Vim

Vim users can configure Vim to use `toml-fmt` as an external formatter for `.toml` files. First you need to install:

```
cargo install toml-fmt
```

Then set `formatprg` inside Vim:

```
set formatprg=toml-fmt
```

Then you can define this command to format the current file.

```
command! FormatFile normal! ggVGgq
```

# Limitations

Currently the formatting is kind of ugly and verbose as there are few formatting options available upstream. If you have a request for a fancy formatting options, please open an issue at [`toml`](https://github.com/alexcrichton/toml-rs).
