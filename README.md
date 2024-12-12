## Spoofing Hash of an Image
A Rust program that creates imperceptible modifications to images to achieve desired cryptographic hash prefixes while preserving visual appearance.

The spoof files arguments are pathfiles and they should be accessible from the terminal in a png file format. This image spoofing program is specific to the 
Portable Network Graphics(PNG) file format. You are recommended to test it out against PNG files.

# How it works
1. Reading the PNG File:
    The program reads the binary data of a PNG file, ensuring that the structure remains valid and adheres to the PNG specification.

2. Appending a Custom Chunk:

    A new chunk is added after the IEND chunk. This chunk contains custom data and is carefully constructed to preserve the integrity of the PNG file.
    The chunk format includes:
        Length: The size of the data (excluding the type and CRC).
        Type: A four-character code indicating the chunk type.
        Data: The custom data, which includes a nonce in BigEndian byte format used for hash tweaking.
        CRC: A cyclic redundancy check computed for the chunk's type and data.

3. Hash Tuning:

    Using the sha2 crate in Rust, the program computes the hash of the modified file.
    A nonce is iteratively modified within the custom chunk data, and the hash is recalculated until it meets specific criteria (e.g., starts with a certain number of zeros).

4. Output:

    Once the desired hash is achieved, the modified PNG file is saved with the new chunk included.
    The original image remains viewable, but the file now includes additional data influencing its hash.


## Installation Guide
# Prerequisites
1. Open a terminal and run the following command.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Follow the on-screen instruction to complete the installation.

3. After the installtion, restart your terminal and verify Rust,

```bash
rustc --version
```

# Running the project
1. Clone the repository

```bash
git clone https://github.com/mvk25/image-hash-spoof.git
```

2. Navigate to the project directory.

```bash
cd image-hash-spoof/spoof
```

3. Build and run the project.

```bash
cargo run -- spoof 0x24 original.png altered.png
```


and get a file `altered.jpg` such that running the sum on a Linux machine produces output like this:

```
sha512sum altered.png
2448a6512f[...more bytes...]93de43f4b5b  altered.png
```

You can view the image contents of the newly created png file by running this on your Linux terminal
```bash
xdg-open altered.png
```

## Additional Resources
1. [Official Rust Documentation](https://www.rust-lang.org/)
2. [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/)