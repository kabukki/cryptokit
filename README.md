# ![Hackit](./assets/logo.png)

**Hackit** is a hacking and utilities toolkit. There are of course many more efficient and specialized software out there, but this was mainly created for learning purposes to practice security concepts, the Rust language and unit tests.

## Features

- **Encoding**: encode, decode
  - base64
  - uuencoding
  - hexadecimal
  - URL encoding
  - ~~ascii85 ...~~
- **Cryptography**: hash
  - MD5
  - ~~DCC, SHA-1, ...~~
- **Formatting**: prettify, stringify, validate
  - JSON
- **Networking**
  - Traceroute
- **DNS**
  - Resolve
  - ~~Record lookup~~
- **Utilities**
  - strlen

## Usage

The tool is consumed via the command-line interface.

```bash
$ hackit uu decode '...'
```

It can be very conveniently used in combination with `xargs` to chain commands

```bash
$ cat package.json | xargs -0 hackit json compact | xargs -0 hackit md5 hash | xargs hackit strlen
32
```

## Ideas

- Ciphers: Caesar, Vigenere, vernam ...
- Network tools
