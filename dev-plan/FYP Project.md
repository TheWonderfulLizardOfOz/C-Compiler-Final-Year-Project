# 1 Principle Idea
As a base the project will be a C compiler, for a subset of C as doing a compiler for the entire language would be too large of a scope. The compiler will be for a target architecture, this target architecture will be fully chosen later once the the compiler up to the intermediate representation has been developed.

# Developing the Compiler
For initial identifying in how the development will be done, the compilation will be done on the C file [[BasicAdd.c]].

```c
int main() {
	return 1 + 1;
}
```

Below is an image obtained from "Compilers Principles, Techniques and Tools"[^1] detailing the phases of a compiler. These phases will be followed for development.

![[CompilerPhases.png|Phases of a compiler  ]]

## Lexical Analyser
Here the program will be read and the characters will be grouped into lexemes. Where each lexeme will have a token produced from it in the form.
```⟨ token-name, attribute-value ⟩```[^1]
So [[BasicAdd.c]] will become:
```
⟨ int ⟩ ⟨ id, 1 ⟩ ⟨ { ⟩
	⟨ return ⟩ ⟨ 1 ⟩ ⟨ + ⟩ ⟨ 1 ⟩
⟨ } ⟩
```
## Syntax Analyser

## Semantic Analyser

## Intermediate Code Generator

## Machine-Independent Code Optimiser

## Code Generator

## Machine-Dependent Code Optimiser

# References
[^1]: Aho, Lam, Sethi, Ullman, 2014, Compilers Principles, Techniques, and Tools Second Edition
