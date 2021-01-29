## `tysh` is a typed shell!!

こんなの作りたい

showコマンドの型を見る
```
> :type show
show : [context] --filename:path -> bytes
> show --filename:./hoge.txt
hogehoge
> show --filename:fuga
! type error: `fuga` has type string, but expected of type path
> show
! execution error: operation `show`: [context] path -> bytes is not executable
```

`[context]` は暗黙的に渡されるパラメータで，環境変数とかカレントディレクトリなどの情報を表す.

