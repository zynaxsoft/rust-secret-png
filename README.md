# Secret PNG

A project that is made by following the challenges in [PNGme](https://picklenerd.github.io/pngme_book/introduction.html). The goal is to create a cli application that can encode/decode messages in a PNG file.

# Usage

1. clone this repo.
2. cargo build --release
3. use the binary located in target/release/rust-secret-png

# Command Arguments

There are 4 sub commands
1. encode PNG_PATH -t CHUNK_TYPE -m MESSAGE [-d DESTINATION] 
    The application will put the message MESSAGE in the png file located at PNG_PATH in a chunk type CHUNK_TYPE
    Note: CHUNK_TYPE is a 4 alphabet ascii character. The third one must be an uppercase letter, e.g. RuSt.
3. decode PNG_PATH -t CHUNK_TYPE
    Find the secret message that is located in a chunk type CHUNK_TYPE.
4. remove PNG_PATH -t CHUNK_TYPE
    Remove an entire chunk of the chunk type CHUNK_TYPE. This can be used to delete the secret message.
5. print
    Print the everything in the png file.

## Example

`rust-secret-png encode mars.png -t RuSt -m "This is a secret message."`

```
Encoding mars.png
Inserted secret message
Done!
```

`rust-secret-png decode output.png -t RuSt`

```
Decoding output.png
The message is: This is a secret message.
Done!
```

`rust-secret-png remove output.png -t RuSt`

```
Removing a chunk type RuSt in output.png
Done!
```

`rust-secret-png decode output.png -t RuSt`

```
Decoding output.png
No hidden message found for the chunk type RuSt
Done!
```
