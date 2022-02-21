---
sidebar_position: 2
---

# Examples

Note: these are for the currently supported syntax and may change

## Batman Theme

```ts
16 of "na";

2 of match {
  <space>;
  "batman";
}

// 🦇🦸‍♂️
```

Turns into

```ts
/(?:na){16}(?:\sbatman){2}/
```

## Twitter Hashtag

```ts
"#";
some of <word>;

// #melody
```

Turns into

```ts
/#\w+/
```

## Introductory Courses

```ts
some of <word>;
<space>;
"1";
2 of <digit>;

// classname 1xx
```

Turns into

```ts
/\w+\s1\d{2}/
```

## Indented Code (2 spaces)

```ts
some of match {
  2 of <space>;
}

some of char;
";";

//  let value = 5;
```

Turns into

```ts
/(?:\s{2})+.+;/
```
