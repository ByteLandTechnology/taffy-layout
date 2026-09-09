# DetailedGridTracksInfo

```ts
type DetailedGridTracksInfo = {
  explicitTracks: number;
  gutters: number[];
  negativeImplicitTracks: number;
  positions: Line<number>[];
  positiveImplicitTracks: number;
  sizes: number[];
};
```

Information about grid tracks (rows or columns).

Tracks use logical order: left-to-right for LTR columns, right-to-left for
RTL columns, and top-to-bottom for rows.

## Properties

| Property                                                     | Type                            | Description                                                  |
| ------------------------------------------------------------ | ------------------------------- | ------------------------------------------------------------ |
| <a id="explicittracks"></a> `explicitTracks`                 | `number`                        | Number of explicitly defined tracks                          |
| <a id="gutters"></a> `gutters`                               | `number`[]                      | Physical spacing between tracks, including content alignment |
| <a id="negativeimplicittracks"></a> `negativeImplicitTracks` | `number`                        | Number of implicit tracks before explicit tracks             |
| <a id="positions"></a> `positions`                           | [`Line`](Line.md)\<`number`\>[] | Physical start/end coordinates of each track                 |
| <a id="positiveimplicittracks"></a> `positiveImplicitTracks` | `number`                        | Number of implicit tracks after explicit tracks              |
| <a id="sizes"></a> `sizes`                                   | `number`[]                      | Array of track sizes (in pixels)                             |
