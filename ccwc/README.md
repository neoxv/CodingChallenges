
# wc Tool - Coding Challenges - 1

This is a rust implementation of the [challenge](https://codingchallenges.fyi/challenges/challenge-wc/) from Coding Challenge to build a replica of the Unix command line tool wc!


## Features

- `-c` flag for total number of bytes.
- `-l` flag for total number of lines.
- `-w` flag for total number of words.
- `-m` flag for total number of characters.


## Deployment

To deploy this challenge run the following commands:

```bash
  git clone https://github.com/neoxv/CodingChallenges.git
```
  
```bash
  cd CodingChallenges/ccwc
```


```bash
  cargo build
```

```bash
  cargo run -- -c ./src/test.txt
  cargo run -- -l ./src/test.txt
  cargo run -- -w ./src/test.txt
  cargo run -- -m ./src/test.txt
  cargo run -- ./src/test.txt
  cat ./src/test.txt | cargo run -- -- -l
```