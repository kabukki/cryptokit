# 🧰 Hackit

> Hacking toolkit and utilities.

There are of course many more efficient and specialized software out there, but this was mainly created for learning purposes.

## Features

### Encodings

| Name          | Encode    | Decode    |
|---------------|-----------|-----------|
| base64        | ✅        | ✅        |
| uuencoding    | ✅        | ✅        |
| ascii85       | ❌        | ❌        |
| hex           | ✅        | ✅        |

### Cryptography

| Name      | Hash      | Crack |
|-----------|---------|---------|
| MD5       | ✅        | ❌     |
| DCC       | ❌        | ❌     |

### Ciphers

Ideas: Caesar, Vigenere, ...

### Network

...

### Formatting

| Name      | Pretty    | Compact   |
|-----------|-----------|-----------|
| JSON      | ✅        | ✅        |

### Miscellaneous

- **strlen**

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
