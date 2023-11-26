| Changeable      | Element | Length | 
|-----------------|---------|--------|
| ListView        | ×       | ×      |
| List            | ×       | √      |
| MutableListView | √       | ×      |
| MutableList     | √       | √      |

```rust
// Lists
type List<T = Any> = LinkedList<T>;
type ListView<T = Any> = &[T; (start, end, step)];
type MutableList<T = Any> = LinkedList<&mut T>;
type MutableListView<T = Any> = &[&mut T; (start, end, step)];
// Arrays
type Array<T = Any> = Vec<T>;
type ArrayView<T = Any> = &[T; (start, end, step)];
type MutableArray<T = Any> = Vec<&mut T>;
type MutableArrayView<T = Any> = &[&mut T; (start, end, step)];
// Map
type IndexMap<K = Any, V = Any> = IndexMap<K, V>;
type IndexMapView<K = Any, V = Any> = &IndexMap<K, V>;
type MutableIndexMap<K = Any, V = Any> = IndexMap<K, &mut V>;
type MutableIndexMapView<K = Any, V = Any> = &IndexMap<K, &mut V>;
```