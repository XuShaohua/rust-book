# for 循环

`for .. in` 表达式用于遍历一个循环体.

```rust, no_run
for i in 1..101 {
...
}
```

它还有一个变体, 用于包含超始值及终止值:

```rust, no_run
for i in 1..=100 {
...
}
```

默认情况下, for 在遍历一个集合时会使用 `Iterator` trait 的 `into_iter()` 方法.
除了这个方法之外, 还有另外两个方法:

* `iter()` 以引用的方法遍历集合, 不改变集合中的值, 该容器接下来还可以被使用
* `into_iter()` 从集合中解析出里面的数据, 一旦遍历完它, 该集合接下来不可再被使用,
  相当于把这个集合 `move` 到了这个循环中
* `iter_mut()` 以可变引用的方法遍历集合, 可以改变集合中的值, 该集合在接下来还可被使用