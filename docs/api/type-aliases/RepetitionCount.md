# RepetitionCount

```ts
type RepetitionCount = number | "auto-fill" | "auto-fit";
```

Grid track repetition parameter.

Defines how many times a track pattern should repeat.

## Remarks

- `number`: Exact number of repetitions (e.g. `repeat(3, ...)`).
- `"auto-fill"`: Fills the container with as many tracks as possible.
- `"auto-fit"`: Fills the container, collapsing empty tracks.
