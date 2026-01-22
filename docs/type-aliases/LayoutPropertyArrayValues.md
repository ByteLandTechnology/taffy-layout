[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / LayoutPropertyArrayValues

# Type Alias: LayoutPropertyArrayValues\<Keys\>

```ts
type LayoutPropertyArrayValues<Keys> = {
  [K in keyof Keys]: Keys[K] extends LayoutProperty
    ? LayoutPropertyValues[Keys[K]]
    : unknown;
};
```

Helper type to convert an array of layout property keys to an array of their value types.

## Type Parameters

| Type Parameter                                           |
| -------------------------------------------------------- |
| `Keys` _extends_ [`LayoutProperty`](LayoutProperty.md)[] |
