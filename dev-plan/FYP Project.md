# 1 Principle Idea
As a base the project will be a C compiler, for a subset of C as doing a compiler for the entire language would be too large of a scope. The compiler will be for a target architecture, this target architecture will be fully chosen later once the the compiler up to the intermediate representation has been developed.

# Language


# 2 Developing the Compiler
For initial identifying in how the development will be done, the compilation will be done on the C file [[BasicAdd.c]].

```c
int main() {
	return 1 + 1;
}
```

Below is an image obtained from "Compilers Principles, Techniques and Tools"[^1] detailing the phases of a compiler. These phases will be followed for development.

![[CompilerPhases.png|Phases of a compiler  ]]

## 2.1 Lexical Analyser
**Token** - A pair with token name and an optional attribute  ```⟨ token-name, attribute-value ⟩```
**Pattern** - Description of the form that the lexemes of a token may take
**Lexeme** - Sequence of characters that match the pattern of a token

### Output of the Lexer

### 2.1.1 Converting from Lexemes to Tokens
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

### Errors during lexical Analysis
At this stage the lexer will be unable to identify some errors as if e.g. `return`is mispelt as `retrun` the lexer will take it as an `id` instead. This kind of error will be handled by the parser instead. 

However, there may be cases where the lexer will be unable to match a lexeme to a pattern e.g. the lexeme is `*!` in this case there are 4 options available[^1]:
1. Delete one character from the remaining input
2. Insert a missing character into the remaining input
3. Replace a character by another character into the remaining input
4. Transpose two adjacent characters.

A common strategy is to find the smallest number of operations needed to continue, however this can take a long time. (Could potentially expand on this and see if I can make my own version of this).

Match to error token?
### 2.1.2 Input Buffering
When building a lexer it is important to look ahead when analysing lexemes so that "\==" doesn't get matched to "=".

### 2.1.3 Other Jobs the Lexical Analyser may do
The lexical analyser is part of the compiler so will need to do extra jobs befor compiling such as:
- Stripping out comments and whitespace
- Keeping track of the number of new lines, so that a compiler error can be asspcoated to a line
	- One way to do this is to keep a copy of the source so that error messages can be inseted into the copy

These are called "Scanning" and are part of the process, this is done before lexical analysis

### Symbol Table
A symbol table is a datastructure storing information about the symbol. Symbol tables usually need to support the same identifier appearing multiple times. Each scope will have it's own declared symbol table. 

Some approaches seem to begin to implement the symbol table in the lexical analysis phase[^1] however other implemetnations implement the symbol table starting in the Semantic Analysis phase[^2] 

The advantage of doing the symbol table in the Semantic Analysis phase is that there will be less problems with ownership in rust (or I am being lazy and leaving the problem to future me).

#### Scopes
Each scope will have its own symbol table - how can I keep track of this

### Symbol Tables in Rust
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
### Representing a Token in Rust
The type of the Token should be an enum and the token itself should be stored in the struct, the maybe type may be worth considering. 

The `AttributeValue` should be a `union` because a Token can only be of one type therefore the `AttributrValue` shoulf only be of one tyoe (obviously).
```rust
enum TokenCategory {
	NUMBER,
	RETURN,
	IDENTIFIER,
	KEYWORD_INT,
	PLUS,
	SEMI_COLON,
	LEFT_CURLY_BRACE,
	RIGHT_CURLY_BRACE,
}

union AttributeValue {
	int_val : i32,
}

struct Token {
	category : TokenCategory,
	attribute_value : Option<AttributeValue>,
	symbole_table_entry : Option<&SymbolTableEntry> // Only if using symbol table in 
}
```

Also shared reference 
## 2.2 Syntax Analyser

## 2.3 Semantic Analyser

## 2.4 Intermediate Code Generator

## 2.5 Machine-Independent Code Optimiser

## 2.6 Code Generator

## 2.7 Machine-Dependent Code Optimiser

# 3 References
[^1]: Aho, Lam, Sethi, Ullman, 2014, Compilers Principles, Techniques, and Tools Second Edition
[^2]: Andrew W.Appel, 1997, Modern Compiler Implementation in C
