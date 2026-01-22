[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / StylePropertyArrayValues

# Type Alias: StylePropertyArrayValues\<Keys\>

```ts
type StylePropertyArrayValues<Keys> = {
  [K in keyof Keys]: Keys[K] extends StyleProperty
    ? StylePropertyValues[Keys[K]]
    : unknown;
};
```

Helper type to convert an array of property paths to an array of their value types.
Unlike `TupleToStyleValues`, this returns an array type instead of a tuple.

## Type Parameters

| Type Parameter                                         |
| ------------------------------------------------------ |
| `Keys` _extends_ [`StyleProperty`](StyleProperty.md)[] |
