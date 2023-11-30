首先阐述以下概念

- 等价: Equivalent, 路径 `"name.vk"` 和 `"Name.vk"` 等价, 但可区分.
- 相等: Equal, 路径 `"name.vk"` 和 `"name.vk"` 相等且不可区分.

unite partial_ordering {
Less,
Equivalent,
Greater,
Unordered,
}

unite weak_order {

std::weak_ordering::less、std::weak_ordering::equivalent 和 std::weak_ordering::greater
}

CompareOrder <=> (const <typename>&) const = default;
bool operator == (const <typename>&) const = default;

std::partial_ordering偏序关系，即存在不可比较的对象。对于可比较的对象，结果的“相等”实际上是“等价” (equivalent) 而非真正的
“相等” (equal)。这里的“相等”指既等价又可互相替换，也就是说若 a = b，则对于任意函数 f 有 f(a) = f(b)。而 partial_ordering
的“相等”不保证可替换性。例如，假设我们定义二维平面上的两个点等价当且仅当它们到原点的欧氏距离相等，那么单位圆上的点都等价，但并不相等，因为它们对于
“取 x 坐标” 这个函数并不都可替换。在基础类型以及标准库类型中，只有浮点数参与比较时，结果才可能是 partial_ordering，因为 NaN
与任何值都不可比较。除此之外也存在等价但不相等的值，如 +0.0 == -0.0，但它们对于取符号函数的结果不同（<cmath> 中的 signbit()
函数）。partial_ordering 的可能取值包括 std::partial_ordering::less、std::partial_ordering::equivalent、 std::
partial_ordering::greater 和 std::partial_ordering::unordered（表示不可比，此时除了 != 比较为 true 外，其他比较都为
false）。std::weak_ordering弱序关系，在偏序关系的基础上不存在不可比较的对象，但相等仍不蕴涵可替换性。基础类型与标准库类型的三路比较不会返回这种结果。可能取值包括
std::weak_ordering::less、std::weak_ordering::equivalent 和 std::weak_ordering::greater。std::
strong_ordering大部分基础类型与标准库类型的三路比较都返回这种结果。强序关系，在弱序关系的基础上，相等蕴涵可替换性。可能取值包括
std::strong_ordering::less、std::strong_ordering::equivalent、 std::strong_ordering::greater 和 strong_ordering::equal。其中
equivalent 和 equal 是相等的。

| operator  | dot call   | return type      | overloadable | derived from |
|:---------:|:-----------|:-----------------|:-------------|:-------------|
|  `a < b`  | `a.le(b)`  | [CompareOrder]() |
| `a <= b`  |
| `a !< b`  |
| `a !<= b` |
|  `a > b`  | `a.cmp(b)` | [CompareOrder]() |
| `a >= b`  | `a.cmp(b)` | [CompareOrder]() |
|  `a !> `  | `a.leq(b)` | [CompareOrder]() |
| `a !>= b` | `a.le(b)`  | [CompareOrder]() |

