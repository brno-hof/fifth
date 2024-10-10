# FIFTH
A simple stack-based programming language inspired by FORTH

# Usage
Running binary:
```bash
./fifth ./path/to/file.5th [OPTIONS]
```
Running with cargo:
```bash
cargo run ./path/to/file.5th [OPTIONS]
```
For a list of available options, please run without arguments.

# Hello World in FIFTH
```
push 0 # [NULL]
push 100 # d
push 108 # l
push 114 # r
push 111 # o
push 87 # W
push 32 # [SPACE]
push 111 # o
push 108 # l
push 108 # l
push 101 # e
push 72 # H
print_string
halt

print_string:
  if
    print_char
    print_string
  then
  return
```

# Syntax
Every keyword is written on a new line.
Indentation is not necessary, but recommended for readability.
## Keywords
```
# this is a comment

# pushes an unsigned 8 bit integer on the stack
push 42

# pops top element
pop

# duplicates top element
# [0][1] -> [0][1][1]
dup

# swaps top two elements
# [0][1] -> [1][0]
swap

# rotates top three elements
# [0][1][2] -> [1][2][0]
rotate

# pushes the element second from the top
# [0][1] -> [0][1][0]
over

# pushes nth element from the top
# pick 1 === dup
# pick 2 === over
pick 42

# pops the top two elements and pushes their sum
# [42][7] -> [49]
add

# pops the top two elements and pushes their difference
# [42][7] -> [35]
sub

# all arithmetic operations work with overflows (255 + 1 = 0), (1 - 3 = 254)

# pops top element and prints it as a byte (here 72)
push 72
print_byte

# pops top element and prints it as an ascii character (here 'H')
push 72
print_char

# halts the program
halt

# the program also halts when reaching end-of-file
```

## Conditional Branching
```
if
  # code here executed if top of stack is greater then zero
then

if
  # code here executed if top of stack is greater than zero
else
  # code here executed if top of stack is less than or equal to zero
then

# there are no loops
# similar behaviour can be achieved by using recursive subroutines (see next section)
```

## Subroutines
```
# this defines a subroutine called "mul"
# it can be called from anywhere in the program, even recursively
# it is good practice to annotate "argument(s)" and "return value(s)" of a subroutine, since these are not obvious from the context

# n1 n2 -> (n1*n2)
mul:
  if # n1 > 0
    push 1
    sub
    swap
    dup
    rotate
    mul # recursive call
    add
  else # n1 = 0
    swap
    pop
  then
return

# subroutines take their "arguments" from the top of the stack
push 3
push 4
mul # this calls the subroutine
halt
```
More code examples are provided in the examples folder
