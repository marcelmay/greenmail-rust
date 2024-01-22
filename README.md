greenmail <-> rust email read/write
===================================

```shell
docker compose up
cargo test test_send_mail -- --nocapture
cargo test test_read_mail -- --nocapture
```
