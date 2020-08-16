# web notes

a small CRUD application using actix, diesel, and tera.

## setup:

```console
$ cargo install diesel_cli --features sqlite
$ diesel setup
$ diesel migration run
$ cargo run
```

then browse [localhost:8080](http://localhost:8080)
