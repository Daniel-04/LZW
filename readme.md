# LZW encoding and decoding in Rust
Raw bytes are encoded as 64bit codes and vice-versa

## Usage examples
```sh
$ cat file.txt | lzw encode > file.lzw
```
```sh
$ cat file.lzw | lzw decode > file.txt
```
```sh
$ lzw encode file.txt > file.lzw
```
```sh
$ lzw decode file.lzw > file.txt
```

## Consider
- The entire input and output are stored in memory, the program is limited by RAM
- To achieve proper compression, the 64bit codes output should be prefix coded
