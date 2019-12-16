# redis-t-digest

Efficient percentile estimation of streaming or distributed data

## Usage

```
$ git clone git@github.com:poga/redis-t-digest.git
$ cd redis-t-digest
$ cargo build
$ redis-server --loadmodule target/debug/libredis_t_digest.so
```

## Commands

### TDIGEST.MERGE

```TDIGEST.MERGE <key> values...```

response: len(values)

### TDIGEST.GET

 ```TDIGEST.GET KEY percentile```

 response: estimated value of percentile