# Spoofing Hash of an Image
Build a tool that takes an image and an arbitrary hexstring and outputs an adjusted file that displays identically to the human eye (when opened in image viewers) but has a hash that begins with the given hexstring.

It should work in such a way that we can run, e.g.

```bash
spoof 0x24 original.jpg altered.jpg
```

and get a file `altered.jpg` such that running the sum on a Linux machine produces output like this:

```
sha512sum <image_name>.jpg
2448a6512f[...more bytes...]93de43f4b5b  <image_name>.jpg
```

