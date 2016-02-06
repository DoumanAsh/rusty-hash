# rusty-hash

Simple utility for checksum calculation

```
Usage: rusty_hash [-c | -o | -p] [algorithms] <input>...

Algorithms:
    --md5       Enables md5 calculation.
    --sha[num]  Enables sha calculation. num can be [1, 256, 512]

Mode:
    Mutually exclusive.
    -c --check  Verifies checksum from files.
    -o --output Write calculations into files with corresponding extension.
    -p --print  Prints checksums to stdout. Default.
```
