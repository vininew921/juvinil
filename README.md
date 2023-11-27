# JUVINIL Programming Language

## :memo: Description
#### Custom Programming Language Compiler

## :wrench: Technologies
#### Rust
![](https://www.rust-lang.org/static/images/rust-logo-blk.svg) 

## :rocket: Run project
First install Rust at [rustup](https://rustup.rs/) ;

Once installed, clone this repository, open it in a terminal, and execute the following command:
```
cargo.exe 'run' '--package' 'juvinil' '--bin' 'juvinil'
```

## :books: Features:
* <b>Lexical Analyzer</b>: Breaks down the source code of a programming language into a sequence of tokens for further processing by the compiler. 
The lexical analyzer scans the source code word by word and groups them into tokens by spliting it by empty spaces. It groups these tokens by rules which define the syntax of the programming language, including keywords, operators, and other language constructs.<br>
**[Token Types](https://github.com/vininew921/juvinil/blob/main/SOURCE_LANGUAGE.md):** Tokens represent meaningful units in the source code, such as keywords (if, else, while), identifiers (variable names), literals (numeric or string constants), and symbols (operators, punctuation).<br>
The lexical analyzer produces a stream of tokens, which is then passed to the next stage of the compiler or interpreter for further analysis and processing.

* <b>Syntax Analyzer</b>: A syntax analyzer, also known as a parser, is a component of a compiler or interpreter. Its main task is to analyze the sequence of tokens produced by the lexical analyzer and determine whether it conforms to the grammatical rules of the programming language. In other words, it checks whether the arrangement of tokens follows the syntax specified by the language [grammar](https://github.com/vininew921/juvinil/blob/main/SOURCE_LANGUAGE.md):.
