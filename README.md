# JUVINIL Programming Language


## :clipboard: TO DO

- ~~Factors should also be able to be an ID instead of a raw number or function call~~ ✔️
- ~~Check if variables have been declared before assigning~~ ✔️
- ~~Check if functions have been declared before calling~~ ✔️
- ~~Make function parameters be registered in the function scope~~ ✔️
- ~~Intermediary code generation to C++, compilation and execution~~ ✔️
- ~~Check parameter cound when calling function~~ ✔️
- ~~Assign STRING to a variable of another type causes an error~~ ✔️
- Type check variables and parameters 

## :memo: Description
#### Custom Programming Language Compiler

## 🔧 Dependencies
You need Rust and g++ to run the project.

Install Rust at [rustup](https://rustup.rs/)

Install g++ at [MinGW](https://www.mingw-w64.org/)

Make sure that Rust is correctly installed by running
```
rustc --version
```

Make sure that g++ is correctly installed by running
```
g++ --version
```

## 🚀 Running

Once installed, clone this repository, open it in a terminal, and execute the following command:
```
cargo run
```

This will compile and run the code located in the `test_inputs/test.jv` file


## :books: Features:
* <b>Lexical Analyzer</b>: Breaks down the source code of a programming language into a sequence of tokens for further processing by the compiler. 
The lexical analyzer scans the source code word by word and groups them into tokens by spliting it by empty spaces. It groups these tokens by rules which define the syntax of the programming language, including keywords, operators, and other language constructs.<br>
**[Token Types](https://github.com/vininew921/juvinil/blob/main/SOURCE_LANGUAGE.md):** Tokens represent meaningful units in the source code, such as keywords (if, else, while), identifiers (variable names), literals (numeric or string constants), and symbols (operators, punctuation).<br>
The lexical analyzer produces a stream of tokens, which is then passed to the next stage of the compiler or interpreter for further analysis and processing.

* <b>Syntax Analyzer</b>: A syntax analyzer, also known as a parser, is a component of a compiler or interpreter. Its main task is to analyze the sequence of tokens produced by the lexical analyzer and determine whether it conforms to the grammatical rules of the programming language. In other words, it checks whether the arrangement of tokens follows the syntax specified by the language [grammar](https://github.com/vininew921/juvinil/blob/main/SOURCE_LANGUAGE.md)
