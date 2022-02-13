# RRX

RRX (Readable Regular Expressions) is a language designed to compile to and maintain a 1-1 relationship with regular expressions, while being more readable and maintainable. 

## Examples

```
16 of capture melody {
  na;
}
2 of capture {
  <space>;
  batman;
}
```

Turns into

```
/(?<melody>na){16}(\sbatman){2}/
```
