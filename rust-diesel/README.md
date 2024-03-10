# Diesel
RustのORマッパー

# 概要
Rustのための安全で拡張性のあるORM兼クエリビルダー
stable rustで動く⇒つまり安定版で動かせる（majorリリースされてる）クレート

Active Record（例：sqlxなど）と異なり、抽象化によってデザインされているため、メンテナンスのしやすい（再利用可能なコードスニペット）実装ができる

## できること
■SQLの抽象化
.net(C#) のEntity Frameworkに類似した、
エンティティにたいしてメソッドチェーンでデータの操作を実現する事ができる。
```
prod_master::table.load(&con);
Post::belonging_to(prod_master).load(&con);
```

■derive属性によるエンティティへのマッピング
```
#[derive(Queryable)] // DB - Entityのマッピングが自動でされる
pub struct prod_master {

}
```
こうすると上述したSQLの抽象化がこのstructに対してそのまま利用できる。（DBからRustのstructに変換する実装が不要）

# 導入

**Diesel CLI**
CLIツールが用意されており、これを用いてmigrationを実行可能

```
// postgres関連のドライバだけインストールする。（cargo install diesel_cliだと全部インストールされるので、特定のDBのドライバがないとエラーになる）
cargo install diesel_cli --no-default-features --features postgres
```
※実行環境に[libpq]ライブラリがインストールされている必要がある.


## 事前準備
dieselに対して、どこにDBがあるかを教える必要がある。
一般的にこれは.envファイルで管理する（のでRust側にはdotenvでの読み込みが必要になる）
※別に環境変数に直接定義（setコマンドとか）しておけばそれでもいい。まあ普通はやらない

```
echo DATABASE_URL=postgres://username:password@localhost/dbname > .env
```

# Migration
マイグレーション機能を実行する
```

```


