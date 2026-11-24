# GridTemplateRepetition

```ts
type GridTemplateRepetition = {
  count: RepetitionCount;
  lineNames?: string[][];
  tracks: TrackSizingFunction[];
};
```

Grid track repetition definition.

When `lineNames` is provided, its length must be `0` or `tracks.length + 1`
(with `tracks.length` clamped to `65535`). Repetitions with any other
`lineNames` length are ignored.

## Properties

| Property                            | Type                                              | Description                                                              |
| ----------------------------------- | ------------------------------------------------- | ------------------------------------------------------------------------ |
| <a id="count"></a> `count`          | [`RepetitionCount`](RepetitionCount.md)           | -                                                                        |
| <a id="linenames"></a> `lineNames?` | `string`[][]                                      | Line-name sets per boundary; must have length `0` or `tracks.length + 1` |
| <a id="tracks"></a> `tracks`        | [`TrackSizingFunction`](TrackSizingFunction.md)[] | -                                                                        |
