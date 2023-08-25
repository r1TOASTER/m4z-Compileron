# m4z Language Compiler - Parser and Tokenizer

The m4z Language Compiler is a project aimed at creating a compiler for the new programming language called "m4z". This compiler includes a tokenizer to convert source code into tokens and a parser to transform tokens into an executeable.

## Language Overview

m4z is a programming language I have designed for fun. A file using the m4z can be viewed to see the language keywords and usage.

## Getting Started

To use the m4z compiler, follow these steps:

1. Clone the repository to your local machine: `git clone https://github.com/r1TOASTER/m4z-Compileron.git`

2. Navigate to the project directory: `cd m4z-Compileron`

3. Build the compiler: `cargo build`

4. Compile a m4z source file: `.\target\debug\m4z.exe source.m4z`


## Tokenizer

The tokenizer is responsible for breaking down the source code into tokens. Tokens are the smallest units in the language, such as keywords, identifiers, operators, and literals.

## Parser

The parser takes the tokens produced by the tokenizer and constructs an executable from it.

## License

This project is licensed under the MIT License.