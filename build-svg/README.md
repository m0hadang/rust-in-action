# build-svg

### command

```chatinput
$ echo 'HELLO' | sha1sum | cut -f1 -d' '
a8eec30a5b2d71bc890175f5b361ebb28d7c54a8
$ cargo run -- $(echo 'HELLO' | sha1sum | cut -f1 -d' ')
```