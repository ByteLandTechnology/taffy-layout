# StylePropertyArrayValues\<Keys\>

```ts
type StylePropertyArrayValues<Keys> = {
  [K in keyof Keys]: Keys[K] extends StyleProperty
    ? StylePropertyValues[Keys[K]]
    : unknown;
};
```

Maps property keys to value types, preserving tuple or array structure.

## Type Parameters

| Type Parameter                                         |
| ------------------------------------------------------ |
| `Keys` _extends_ [`StyleProperty`](StyleProperty.md)[] |
