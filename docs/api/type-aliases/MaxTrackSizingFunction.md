# MaxTrackSizingFunction

```ts
type MaxTrackSizingFunction =
  | number
  | `${number}%`
  | `${number}fr`
  | "auto"
  | "min-content"
  | "max-content"
  | "fit-content";
```

Maximum track sizing function.

Defines the maximum size of a grid track.
The supported `"fit-content"` token uses a zero-pixel fit-content limit.
Parameterized strings such as `"fit-content(100px)"` are not supported.
