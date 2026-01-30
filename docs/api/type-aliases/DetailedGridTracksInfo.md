# DetailedGridTracksInfo

```ts
type DetailedGridTracksInfo = {
  explicitTracks: number;
  gutters: number[];
  negativeImplicitTracks: number;
  positiveImplicitTracks: number;
  sizes: number[];
};
```

Information about grid tracks (rows or columns).

Provides detailed sizing and gutter information for a set of grid tracks.

## Properties

| Property                                                     | Type       | Description                                      |
| ------------------------------------------------------------ | ---------- | ------------------------------------------------ |
| <a id="explicittracks"></a> `explicitTracks`                 | `number`   | Number of explicitly defined tracks              |
| <a id="gutters"></a> `gutters`                               | `number`[] | Array of gutter sizes between tracks (in pixels) |
| <a id="negativeimplicittracks"></a> `negativeImplicitTracks` | `number`   | Number of implicit tracks before explicit tracks |
| <a id="positiveimplicittracks"></a> `positiveImplicitTracks` | `number`   | Number of implicit tracks after explicit tracks  |
| <a id="sizes"></a> `sizes`                                   | `number`[] | Array of track sizes (in pixels)                 |
