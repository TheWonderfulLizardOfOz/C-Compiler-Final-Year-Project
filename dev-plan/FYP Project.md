---
number headings: auto, first-level 1, max 6, 1.1
---
# 1 Principle Idea
As a base the project will be a C compiler, for a subset of C as doing a compiler for the entire language would be too large of a scope. The compiler will be for a target architecture, this target architecture will be fully chosen later once the the compiler up to the intermediate representation has been developed.

# 2 Language


# 3 Developing the Compiler
For initial identifying in how the development will be done, the compilation will be done on the C file [[BasicAdd.c]].

```c
int main() {
	return 1 + 1;
}
```

Below is an image obtained from "Compilers Principles, Techniques and Tools"[^1] detailing the phases of a compiler. These phases will be followed for development.

![[CompilerPhases.png|Phases of a compiler  ]]

## 3.1 Lexical Analyser
**Token** - A pair with token name and an optional attribute  ```⟨ token-name, attribute-value ⟩```
**Pattern** - Description of the form that the lexemes of a token may take
**Lexeme** - Sequence of characters that match the pattern of a token

### 3.1.1 Output of the Lexer

### 3.1.2 Converting from Lexemes to Tokens
Here the program will be read and the characters will be grouped into lexemes, a lexeme is a sequence of characters that match the pattern for a token. Each lexeme will have a token produced from it in the following form:
```⟨ token-name, attribute-value ⟩```[^1] 
The attribute value is optional.

So [[BasicAdd.c]] will become:
```
⟨ int ⟩ ⟨ id, 0 ⟩ ⟨ { ⟩
	⟨ return ⟩ ⟨ number, 1 ⟩ ⟨ + ⟩ ⟨ number, 1 ⟩
⟨ } ⟩
```

When a pattern can have multiple lexemes matching to it, an attribute is used as additional information. An example of this is the pattern for a function name or a variable name, e.g.  the ```id``` where the attribute is a pointer to an entry in the symbol table, or ```number```where the attribute is the value of the number.

### 3.1.3 Errors during lexical Analysis
At this stage the lexer will be unable to identify some errors as if e.g. `return`is mispelt as `retrun` the lexer will take it as an `id` instead. This kind of error will be handled by the parser instead. 

However, there may be cases where the lexer will be unable to match a lexeme to a pattern e.g. the lexeme is `*!` in this case there are 4 options available[^1]:
1. Delete one character from the remaining input
2. Insert a missing character into the remaining input
3. Replace a character by another character into the remaining input
4. Transpose two adjacent characters.

A common strategy is to find the smallest number of operations needed to continue, however this can take a long time. (Could potentially expand on this and see if I can make my own version of this).

Match to error token?
### 3.1.4 Input Buffering
When building a lexer it is important to look ahead when analysing lexemes so that "\==" doesn't get matched to "=".

### 3.1.5 Other Jobs the Lexical Analyser may do
The lexical analyser is part of the compiler so will need to do extra jobs befor compiling such as:
- Stripping out comments and whitespace
- Keeping track of the number of new lines, so that a compiler error can be asspcoated to a line
	- One way to do this is to keep a copy of the source so that error messages can be inseted into the copy

These are called "Scanning" and are part of the process, this is done before lexical analysis

### 3.1.6 Symbol Table
A symbol table is a datastructure storing information about the symbol. Symbol tables usually need to support the same identifier appearing multiple times. Each scope will have it's own declared symbol table. 

Some approaches seem to begin to implement the symbol table in the lexical analysis phase[^1] however other implemetnations implement the symbol table starting in the Semantic Analysis phase[^2] 

The advantage of doing the symbol table in the Semantic Analysis phase is that there will be less problems with ownership in rust (or I am being lazy and leaving the problem to future me).

However, if we do this the identifier names still need to be stored so that they can be used in the symbol table later.

#### 3.1.6.1 Scopes
Each scope will have its own symbol table - how can I keep track of this

### 3.1.7 Symbol Tables in Rust
The symbol table could be an array of a structs, some implemntations use hash tables
```Rust
struct SymbolTableEntry {
	// Info here
	cType : CType,
}

struct SymbolTableNode {
	entries : HashMap<String, SymbolTableEntry>,
	parent_scope : Option<&SymbolTymbolTableNode>
}
```

for the type to be something we must have an enum for types that are implemented

```Rust
enum CType {
	INT,
}
```
### 3.1.8 Representing a Token in Rust
The type of the Token should be an enum and the token itself should be stored in the struct, the maybe type may be worth considering. 

The `AttributeValue` should be a `union` because a Token can only be of one type therefore the `AttributrValue` shoulf only be of one tyoe (obviously).
```rust
pub enum TokenCategory {
	NUMBER,
	RETURN,
	IDENTIFIER,
	KEYWORD_INT,
	PLUS,
	SEMI_COLON,
	LEFT_CURLY_BRACE,
	RIGHT_CURLY_BRACE,
}

pub union AttributeValue {
	int_val : i32,
}

pub struct Token {
	category : TokenCategory,
	attribute_value : Option<AttributeValue>,
	symbole_table_entry : Option<&SymbolTableEntry> // Only if using symbol table in 
}
```

Also shared reference 

### 3.1.9 Stream Output vs Array Output
Stream output would return tokens as the program runs with functions to get the next token being called by the parser when needed. Array output would put the tokens into a whole array and then the parser would run using the entire array.

A stream would have less memory usage and if there is an error the parser would not have to parse every line. 

Array could be made for testing purposes

### 3.1.10 Avoiding matching if as an identifier
Keywords such as if and return risk being matched to an identifier instead of their actual token. To avoid this when an alphanumeric character is read the entire

### 3.1.11 How I Actually Implemented the Parser

#### 3.1.11.1 Token Category
The Token Category Enum is the same except that modified it to match Rust standard of CamelCase for enums
```Rust
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TokenCategory {
    Number,
    KeywordReturn,
    Identifier,
    KeywordInt,
    Plus,
    SemiColon,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftBracket,
    RightBracket,
    Eof,
    TokenError, // If token is invalid
}
```
`TokenError` is not currently used as it will be used later for further error handling. `#[derive(Debug, Hash, PartialEq, Eq)]`was used putting the tokens in a hash table for my initial idea of using regex but this is no longer needed.

#### 3.1.11.2 Token Struct
```Rust
#[derive(Debug)]
pub struct Token {
    pub category: TokenCategory,
    pub attribute_value: Option<String>,
}
```
Same except I went with creating the symbol table later on so I removed the pointer to that the attribute_value now contains the lexeme of the identifier or value of a token.

`#[derive(debug)]` Allows the variables of the type `Token` to be printed, mostly useful for debugging.
#### 3.1.11.3 Lexer Implementation

```Rust
pub struct Lexer<'a> {
    input_file: &'a str,
    char_iterator: Peekable<Chars<'a>>,
    // matches: HashMap<TokenCategory, Regex>,
}

impl<'a> Lexer<'a> {
    pub fn new(input_file: &'a str) -> Self {
        let lexer = Lexer {
            input_file,
            char_iterator: input_file.chars().peekable(),
        };

        return lexer;
    }

    pub fn print_input(&mut self) {
        println!("{}", self.input_file);
    }

....
}
```
The Lexer is implemented as a struct allowing for multiple instances of the lexer to be used. The `char_iterator` field is used as a pointer to the current character that is being looked at. The `print_input` function is used for debugging purposes.

```Rust
pub fn get_next_token(&mut self) -> Token {
	use TokenCategory::*;
	let mut cur_lexeme = String::from("");
	let mut token = Token {
		category: TokenError,
		attribute_value: None,
	};

	// find next non whitespace
	while self.char_iterator.peek().map_or(false, |c| c.is_whitespace()) {
		self.char_iterator.next();
	}

	if self.char_iterator.peek().is_none() {
		token.category = Eof;
		return token;
	}

	let first_char = self.char_iterator.next().unwrap();
	cur_lexeme.push(first_char);

	if first_char.is_alphabetic() {
		return self.possible_identifier_state(token, cur_lexeme);
		
	} else if first_char.is_digit(10) {
		return self.number_state(token, cur_lexeme)
	}

	match cur_lexeme.as_str() {
		"}" => token.category = RightCurlyBrace,
		"{" => token.category = LeftCurlyBrace,
		";" => token.category = SemiColon,
		"+" => token.category = Plus,
		")" => token.category = RightBracket,
		"(" => token.category = LeftBracket,
		_   => eprintln!("Error! Unexpected token {} on line n", cur_lexeme),                     
	}
	return token;
}
```

Tokens will be sent via a stream and the parser can call `get_next_token`when needed. To determine the `category`of a token a state machine is implemented. To find the start of a token the `char_iterator`will change the character it is pointing at until it reaches a non-whitespace character or `EOF`(when `self.char_iterator.peek()`returns `None`).

Once a non-whitespace character is found we check if this character is alphabetic (this might need to change in case I want it to compile for non latin languages, but also why would I want to do that?) or a digit. If the character is alphabetic the `possible_identifier_state`will be called, or if the character is a digit the `number_state` will be called.

```Rust
fn possible_identifier_state(&mut self, mut token: Token, mut cur_lexeme: String) -> Token {
	while self.char_iterator.peek().map_or(false, |c| c.is_alphanumeric()) {
		cur_lexeme.push(self.char_iterator.next().unwrap());
	}

	match cur_lexeme.as_str() {
		"int" => token.category = TokenCategory::KeywordInt,
		"return" => token.category = TokenCategory::KeywordReturn,
		_ => { 
			token.category = TokenCategory::Identifier;
			token.attribute_value = Some(cur_lexeme);
		},
	}
	
	return token
}
```

The `possible_identifer_state` attempts to finish the rest of the lexeme by pushing to the the lexeme until it finds a character that is not alpha numeric (as an identifier can contain both digits). No error will be returned if it encounters a non alphanumeric character before a whitespace as cases like `x+1` should be valid. Once the full lexeme has been identified we attempt to match the current lexeme to `"int"` or `"return"` and then to the appropriate category. In the case the lexeme matches to neither it will set the `category`to `Identifier` and the `attribute_value` will be set to the lexeme.

It is important to note that by passing the `token` and `cur_lexeme` to this function they are no longer owned by `get_next_token` and are now owned by the current state so it is importent to return `token` so that ownership can be transffered back or immediatly returned. Yay rust ownership that I understand slightly :)

```Rust
fn number_state(&mut self, mut token: Token, mut cur_lexeme: String) -> Token {
	while self.char_iterator.peek().map_or(false, |c| c.is_digit(10)) {
		 cur_lexeme.push(self.char_iterator.next().unwrap());
	}
	if self.char_iterator.peek().map_or(false, |c| c.is_alphabetic()) {
		// Error state : e.g. 1011aaaa is not valid
		cur_lexeme.push(self.char_iterator.next().unwrap());
		eprintln!("Error on line n (line numbers not implemented): {}", cur_lexeme);
		process::exit(1);
	}
	token.category = TokenCategory::Number;
	token.attribute_value = Some(cur_lexeme);
	return token
}
```

Unlike the `possible_identifier_state` the `number_state` assumes that the token will definitly be a number, currently it only checks for alphabet numbers and throws an error if it is, later when floats are involved will check for multiple decimal places.
### 3.1.12 Testing the lexer on BasicAdd.c
The code can be run using `cargo run --bin c_lexer ../examples/BasicAdd.c`

(I have done all these test just don't have a screenshot)
#### 3.1.12.1 Test 1 - Run Correctly on BasicAdd.c
In the image below we can see the output that is as expected.
![[BasicAddLexerRes.png]]

#### 3.1.12.2 Test 2 - `inti`
`inti` should be detected as an `Identifier` and not a `KeywordInt`.

#### 3.1.12.3 Test 3 - `123123a`
The lexer should reject `123123a` as this is not a valid token and therefore should report an error.

#### 3.1.12.4 Test 4 -  `$`
The lexer should reject `$` as it is not a valid lexeme for a token and should therefore report an error.

#### 3.1.12.5 Test 5 - Accept Identifier with Underscore
The lexer should accept an identifier with an undersocre e.g. `main_underscore`.

#### 3.1.12.6 Test 6 - No Space Between Plus
The lexer should read `1+1` as 3 seperate tokens.

## 3.2 Syntax Analyser
The phase where we use the parser.
### 3.2.1 Defining the Grammar
The grammar defined here will be adapted from section A.13 of The C Programming Language (2nd Edition)[^3], and A.3 of ISO working draft. (Note to self should probably refer to this for tokens as well).
```ebnf
(*External definitions*)
<translation-unit> ::= <external-declaration>
<external-declaration> ::= <function-definition> | <declaration>
<function-definition> ::= <declaration-specifiers> <declarator> <function-body>
<function-body> ::= <compound-statement>


(*Expressions*)
<primary-expression> ::= <identifier> | <constant> | ( <expression> )
<postfix-expression> ::= <primary-expression> | <postfix-expression> ( ) 
<unary-expression> ::= <postfix-expression>
<cast-expression> ::= <unary-expression> 
<multiplicative-expression> ::= <cast-expression>
<additive-expression> ::= <multiplicative-expression> | <additive-expression> + <multiplicative-expression> | <cast-expression>

(*Declarators*)
<type-specifier> ::= void | int
<function-declarator> ::= <direct-declarator> ( )

(*Statements*)
<compound-statement> ::= { block-item- list} | { }
<block-item-list> ::= <block-item> | <block-item-list> <block-item>
<block-item> ::= <declaration> | <unlabeled-statement>
<jump-statement> ::= return | return <expression>

```

## 3.3 Semantic Analyser

## 3.4 Intermediate Code Generator

## 3.5 Machine-Independent Code Optimiser

## 3.6 Code Generator

## 3.7 Machine-Dependent Code Optimiser

# 4 References
[^1]: Aho, Lam, Sethi, Ullman, 2014, Compilers Principles, Techniques, and Tools Second Edition
[^2]: Andrew W.Appel, 1997, Modern Compiler Implementation in C
[^3]: Brian Kernighan, Dennis Ritchiem, 1998, The C Programming Language (2nd Edition)
[^4]: JeanHeyd Meneide, Freek Wiedijk, Janurary 2024, Information technology - Programming languages - C