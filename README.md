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

# pushes an unsigned 8 bit integer (0-255) on the stack
push 42
push 255
push 0

# removes topmost byte from the stack
pop

# duplicates topmost byte
# [0][1] -> [0][1][1]
dup

# swaps top two bytes
# [0][1] -> [1][0]
swap

# rotates top three bytes
# [0][1][2] -> [1][2][0]
rotate

# copies the byte second from the top and pushes it
# [0][1] -> [0][1][0]
over

# copies nth byte from the top and pushes it to the stack
# pick 1 === dup
# pick 2 === over
pick 3

# pops the top two bytes and pushes their sum
# [42][7] -> [49]
push 42
push 7
add

# pops the top two bytes and pushes their difference
# [42][7] -> [35]
push 42
push 7
sub

# all arithmetic operations work with overflows (255 + 1 = 0), (1 - 3 = 254)

# pops topmost byte and prints it as a number (here 72)
push 72
print_byte

# pops topmost byte and prints it as an ascii character (here 'H')
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
  # code here executed if top of stack is equal to zero
then

# if-conditions do not pop the topmost element

# there are no loops
# similar behaviour can be achieved by using recursive subroutines
```

## Subroutines
```
# this defines a subroutine called "mul"
# it can be called from anywhere in the program, even recursively
# it is good practice to annotate "argument(s)" and "return value(s)" of a subroutine, since these are not obvious from the context

# n m -> (n*m)
mul:
  if # m > 0
    push 1
    sub
    swap
    dup
    rotate
    mul # recursive call
    add
  else # m = 0
    swap
    pop
  then
return

push 3
push 4
mul # this calls the subroutine
halt
```
More code examples are provided in the examples folder
