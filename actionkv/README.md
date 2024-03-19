# actionkv

usage

```
$ cargo run test.bin insert test-key test-value

$ cargo run test.bin insert hello world

$ cargo run test.bin get test-key
[116, 101, 115, 116, 45, 118, 97, 108, 117, 101]

$ cargo run test.bin get hello
[119, 111, 114, 108, 100]

$ cargo run test.bin update hello 12345
$ cargo run test.bin get hello
[49, 50, 51, 52, 53]

$ cargo run test.bin delete hello
$ cargo run test.bin get hello
[]
```
