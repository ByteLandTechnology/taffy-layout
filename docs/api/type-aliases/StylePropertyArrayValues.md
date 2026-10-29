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

| Type Parameter                                                                          |
| --------------------------------------------------------------------------------------- |
| <a id="type-parameter-keys"></a> `Keys` _extends_ [`StyleProperty`](StyleProperty.md)[] |
