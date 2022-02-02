# rjq

simple `jq` command by Rust.

## Usage

```sh
$ cargo run "{\"field1\":.,\"field2\":.string-field}" data/example.json
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/rjq '{"field1":.,"field2":.string-field}' data/example.json`
{"field1":{"array-field":["first field","next field",{"object-in-array":"string value in object-in-array"}],"nested-field":{"inner-number":100,"inner-string":"inner value"},"string-field":"string value"},"field2":"string value"}
```

## Note

- [Haskell入門 関数型プログラミング言語の基礎と実践 第9章](https://gihyo.jp/book/2017/978-4-7741-9237-6)
- https://github.com/akthrms/haskell_nyumon/tree/main/chapter09/hjq
