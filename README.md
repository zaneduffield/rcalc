# rcalc

A simple command-line calculator implemented in Rust
```
Welcome to rcalc!
You can evaluate math expressions using + - * / ^ ()

>>> 1 + 5*3^2
46
```

where incomplete expressions will flow onto subsequent lines
```
>>> 4 * 
... 5 + 2
22
```

with nice error reporting
```
>>> 5 ** 2

  5 ** 2
     ^ not expected here
>>> 5 & 2

  5 & 2
    ^ unknown symbol
```

<br/>

The parser is implemented using a top-down recursive descent algorithm recognising following 
grammar (ignoring whitespace)
```
E -> T | T + E | T - E
T -> F | F * T | F / T
F -> P | P ^ F
P -> d | (E) | -F
d -> \d+(\.\d+)?
```