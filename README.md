# rjq

simple `jq` command by Rust.

## Usage

```sh
$ rjq query [filename]
```

## Examples

```sh
$ cat data/example.json | target/release/rjq "{\"field1\": ., \"field2\": .string-field}"
{
  "field1": {
    "array-field": [
      "first field",
      "next field",
      {
        "object-in-array": "string value in object-in-array"
      }
    ],
    "nested-field": {
      "inner-number": 100,
      "inner-string": "inner value"
    },
    "string-field": "string value"
  },
  "field2": "string value"
}
```

```sh
$ target/release/rjq "[.string-field, .nested-field.inner-string]" data/example.json
[
  "string value",
  "inner value"
]
```

## Note

- [Haskell入門 関数型プログラミング言語の基礎と実践 第9章](https://gihyo.jp/book/2017/978-4-7741-9237-6)
- https://github.com/akthrms/haskell_nyumon/tree/main/chapter09/hjq
