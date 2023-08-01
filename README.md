バージョンの違いに対応するのは~~面倒なので~~諦め、とりあえず手元のバージョンで動くコードが書ければよしということにする。

templateディレクトリを複製し.vscode/settings.jsonに追記すればいいのだが、initクレートをビルドしておけば以下のようなコマンドでOK。
```sh
make copy NAME=abc000a
```

`cargo test`でテストも実行できる。
