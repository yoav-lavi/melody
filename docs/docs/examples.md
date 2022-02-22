---
sidebar_position: 2
---

# Examples

Note: these are for the currently supported syntax and may change

## Batman Theme

```
16 of "na";

2 of match {
  <whitespace>;
  "batman";
}

// 🦇🦸‍♂️
```

Turns into

```
/(?:na){16}(?:\sbatman){2}/
```

## Twitter Hashtag

```
"#";
some of <word>;

// #melody
```

Turns into

```
/#\w+/
```

## Introductory Courses

```
some of <word>;
<whitespace>;
"1";
2 of <digit>;

// classname 1xx
```

Turns into

```
/\w+\s1\d{2}/
```

## Indented Code (2 spaces)

```
some of match {
  2 of <whitespace>;
}

some of char;
";";

//  let value = 5;
```

Turns into

```
/(?:\s{2})+.+;/
```
