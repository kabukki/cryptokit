# 🧰 <img src="data:png;base64,iVBORw0KGgoAAAANSUhEUgAAAGsAAAAYCAYAAAD9CQNjAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAPoSURBVHgB5VqNddowED78MgAjeIMkEyAmKJ0gZILQCTATlCzQOBO0TIDoAkkniLtA4y4A/c6ciXBsSU5NIpLvPaG/02HfSafTyT0SLJdLhSwfDof35ABo+8hGoE0ddCqKotPNZnPG9V6vd79erxcYl1ELmHyQ/qL84MsnSRIlubbQxHi2wXQ6vbWw4nfokz9yJFOWfeHxUuiT4lfrJTIFQVxW/uAZILgRXuxG/jxtoImFhnnu2rmM9jn+L0V55hJ2HR/UvfhAAfx833ks0iXZsQSfGGMSlIfIsko/C/mO2mFM+7Lk9xjRy6A5RVJR5A/rDBMBF8qXpnKGmQ8+ZhqmPSCfPnm8FxQzRRZLdVGjKMaE2iFDMldpTC9XFCPlnxPqEGweRcAx13nWI5tj5ufSz+0s4EJAyHnmn9fwiT34TEBzJXx41g6pJcREJlLNjHIVKT23IgppKuVLGU8Gryqank9Z+JTQ/NOpsmi79GMuQMBfINy52SnmKoGwSRR2tlqtLgaDwd5eUSrBwWcifJhW1fGxgfcp2pqmHVu05Q3kuqYtrvRn1IzM0h/78omoQ0BwF+WfVgVsAn0Jbc0aK2NcQzLy5DNx8LEhoSdBzRrMX1DoTFniIRbeDgS3ctGDplwFZzV8YqFZePBZ1PGxYTab8WrcTSxxLIJHlyvLdDoyD/q8Zly1npMbmeT3HrSF+YOCE2Osy1MMBp2awTeChvDPlVK+DgY7LsWEgNlOjsH8lejawXh1YN/SvrQVN/3acQgODu9hZXmhhZseLPZWFp9XEBW4of9EV3w6RExPZ5my/igmUNNH8wYDx+7cVgG3jZHu4CFeUOCo7lk/sFn/sg3ggCe5wzhd8ekKpYfJ3iXvU6XnqGjrwvfxvCkURiHvY3vK4jOLRyQ9gaCVjaYrPh2DFfS5Yu5SOWMVoS089xzVhSWS8aZ4N2ZQDtNNyOi5ogpIW3nWYh5tg7avhqNXFisJccEpVumDhUzbHAi569JS/USB4ujPWUAC83XloHGGv4Dfkre5YHxVBLeyJKJe7hnOeB9W1KkUvcJNx4xQzWAheI7iOy4oWZmKyz7B42NHkMqSy0ZGv+kmmL/LkMtLBq/EOR0WvUqy9fXoAAhyz+J4H5QxM26UH/h7C/7ghvuhTEXGNblcUGZ0OMRIX2l/P4uN8reaMXx10+kECtbB4AtK40aZMTY/vhHkoqjUxgs8Hskfm5o2RfZvKFRNW9url42LoFSWlq+EtGtAFEWP6/X6mupnTVs+bO7SJhpRGPcn4kiUDkcml5e77zIaoJFmiEpocuMP01L9e3G0ZUn++El+d3qM0oTfusb8A0rA7ufdujaMAAAAAElFTkSuQmCC" style="vertical-align: middle" />

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

## Ideas

- Ciphers: Caesar, Vigenere, ...
- Network tools
