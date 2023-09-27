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