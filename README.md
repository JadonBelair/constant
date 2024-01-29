# Con(catinated)sta(ck-based)(i)nt(erptreted language)
Constant is a very simple concatinative, stack-based programming language using [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation)

## How to use
run the program passing in a file path to the source code, if no file is provided, a REPL will be started instead

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

// if statements
10 bind x
if x 20 > do // if checks if the top stack item is true
    "this is true" print // these statements only run when the if is true
elif x 15 > do
    "now this is true" print
else do
    "non were true" print
end

// while statements
0 bind x
while x 20 < do // while loops only run while these statements evaluate to true
    x print
    x 1 +
    bind x
end

// variables
bind x // consumes and binds the top value on stack to x
x // pushes the value bound to x to the stack, x does not change
```
