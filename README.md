# transform

Toy project to take a peek on rust programming.

A library for binary transformations with cli. For now (maybe forever) only supports base64 encoding/decoding.

Try it:
```
echo -n "for fun and profit" | cargo run -- --encode --base64
echo -n "Zm9yIGZ1biBhbmQgcHJvZml0" | cargo run -- --base64 -d
```