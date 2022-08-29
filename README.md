# ![Hackit](./assets/logo.png)

**Hackit** is a hacking and utilities toolkit. There are of course many more efficient and specialized software out there, but this was mainly created for learning purposes to practice Go and unit tests.

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

### Formatting

| Name      | Prettify    | Compact | Validate  |
|-----------|-------------|---------|-----------|
| JSON      | ✅          | ✅       | ✅        |

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

## Ideas

- Ciphers: Caesar, Vigenere, ...
- Network tools
