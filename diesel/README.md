# diesel

## Timestampの扱い
[StackOverflowの記事](https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres)

dieselにより生成されているmodelとRust側のデータ構造の適切なマッピングが必要

`diesel::sql_types`はdieselが定義するDatabase向けのデータ構造を表現している

これらを変換するためには主に以下のトレイトを実装する必要がある

- FromSql
- Timestamp
- Pg

単純な解決方法としては
`std::time::SystemTime`を利用する方法があるが容量が少ない

適切な方法としてはchronoクレートの`NaiveDateTime`を利用する方法

dieselが解釈するために、`Cargo.toml`内で、dieselの`features`フラグにchronoを含めてあげる必要がある
```
// 修正前
diesel = {version="2.2.0", features=["postgres"]}
// 修正後
diesel = {version="2.2.0", features=["postgres", "chrono"}
```

