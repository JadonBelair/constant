# Con(catinated)sta(ck-based)(i)nt(erptreted language)
Constant is a very simple concatinative, stack-based programming language using [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation)

## How to use
run the program passing in a file path to the source code, if not file is provided, a REPL will be started instead

## Language features

```
// pushing values to the stack
false "this is a string" 2 3.14159

// math and comparisons
+ - * /
> < >= <= == !=

// built-ins
print //prints the top stack value
dup // duplicates the item on the top of the stack
swap // swaps the top 2 items on the stack
drop // removes the top item from the stack

// variables
bind x // consumes and binds the top value on stack to x
x // pushes the value bound to x to the stack, x does not change
```
