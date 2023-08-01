バージョンの違いに対応するのは~~面倒なので~~諦め、とりあえず手元のバージョンで動くコードが書ければよしということにする。

まずtemplateディレクトリを複製し
```sh
$ cp -r template abc000a
```
.vscode/settings.jsonに追記する。
```diff json
  {
      "editor.indentSize": 4,
      "rust-analyzer.linkedProjects": [
          "template/Cargo.toml",
+         "abc000a/Cargo.toml",
      ]
  }
```

`cargo test`でテストを実行できる。
