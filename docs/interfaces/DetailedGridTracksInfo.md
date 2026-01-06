[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / DetailedGridTracksInfo

# Interface: DetailedGridTracksInfo

Information about grid tracks (rows or columns).

Provides detailed sizing and gutter information for a set of grid tracks.

## Properties

| Property                                                         | Type       | Description                                      |
| ---------------------------------------------------------------- | ---------- | ------------------------------------------------ |
| <a id="explicit_tracks"></a> `explicit_tracks`                   | `number`   | Number of explicitly defined tracks              |
| <a id="gutters"></a> `gutters`                                   | `number`[] | Array of gutter sizes between tracks (in pixels) |
| <a id="negative_implicit_tracks"></a> `negative_implicit_tracks` | `number`   | Number of implicit tracks before explicit tracks |
| <a id="positive_implicit_tracks"></a> `positive_implicit_tracks` | `number`   | Number of implicit tracks after explicit tracks  |
| <a id="sizes"></a> `sizes`                                       | `number`[] | Array of track sizes (in pixels)                 |
