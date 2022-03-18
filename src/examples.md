# Examples

Note: these are for the currently supported syntax and may change

## Batman Theme

```rs
16 of "na";

2 of match {
  <space>;
  "batman";
}

// 🦇🦸‍♂️
```

Turns into

```
(?:na){16}(?: batman){2}
```

## Twitter Hashtag

```rs
"#";
some of <word>;

// #melody
```

Turns into

```
#(?:\w)+
```

## Introductory Courses

```rs
some of <alphabetic>;
<space>;
"1";
2 of <digit>;

// classname 1xx
```

Turns into

```
(?:[a-zA-Z])+ 1(?:\d){2}
```

## Indented Code (2 spaces)

```rs
some of match {
  2 of <space>;
}

some of <char>;
";";

// let value = 5;
```

Turns into

```
(?: {2})+.+;
```

## Semantic Versions

```rs
<start>;

option of "v";

capture major {
  some of <digit>;
}

".";

capture minor {
  some of <digit>;
}

".";

capture patch {
  some of <digit>;
}

<end>;

// v1.0.0
```

Turns into

```
^v?(?<major>(?:\d)+)\.(?<minor>(?:\d)+)\.(?<patch>(?:\d)+)$
```
