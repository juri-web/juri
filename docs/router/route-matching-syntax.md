# 路由的匹配语法

## 普通参数

```rust,noplayground
/// 匹配 /one
router.at("/one");

/// 匹配 /one/two
router.at("/one/two");
```

## 参数

通过加 `:` 来把一个路径节点，变成匹配参数。

```rust,noplayground
/// 匹配 /one/two
/// 通过 `request.param("chapters")` 来获取 two
router.at("/one/:chapters");

/// 匹配 /one/two/three
router.get("/one/:chapters/three", handle_index);
```

## 可重复的参数

参数路径节点后面加 `+` 时可匹配 1 个或多个

```rust,noplayground
/// 匹配 /one/two，/one/two/three 时。
/// /one/two 时 `request.param("chapters") == two
/// /one/two/three 时 `request.param("chapters") == two/three
router.get("/one/:chapters+", handle_index);
```

