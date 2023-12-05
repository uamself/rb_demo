## axum 提取器之参数获取
一、从path中提取参数

```rust
Router::new().route("/user/:id", get(user_info))
// eg: /user/1，将解析出id=1 
async fn user_info(Path(id): Path<i32>) -> String { format!("user id:{}", id) }
```

通过正则匹配获取路由路径中的值，不仅可以获取单个值，也可以匹配多个值
```rust
Router::new().route("/person/:id/:age", get(person))

// eg: /person/12/3，将解析出id=12, age=30
async fn person(Path((id, age)): Path<(i32, i32)>) -> String { format!("id:{},age:{}", id, age) }
```
也可以封装成一个struct进行匹配
```rust
Router::new().route("/path_req/:a/:b/:c/:d", get(path_req))

#[derive(Deserialize)]
struct SomeRequest{
  a: String, 
  b: i32, 
  c: String,
  d: u32, 
} 

// eg: path_req/a1/b1/c1/d1
async fn path_req(Path(req): Path<SomeRequest>) -> String {
   format!("a:{},b:{},c:{},d:{}", req.a, req.b, req.c, req.d)
}
```
