## ref
https://www.youtube.com/watch?v=XZtlD_m59sM&t=462s

### run
```bash
cargo watch -q -c -w src/ -x run
```
### test
```bash
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

### 動作確認
```bash
localhost:8088/hello
```

### 使用中ポートの調べ方
```bash
sudo lsof -P -i:8088
```

### ポートの開け方
```bash
sudo kill 11881  
```