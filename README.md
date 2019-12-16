# redis-percentile

Efficient percentile estimation of streaming or distributed data with [t-digest](https://github.com/MnO2/t-digest) algorithm.

## Usage

```
$ git clone git@github.com:poga/redis-percentile.git
$ cd redis-percentile
$ cargo build
$ redis-server --loadmodule target/debug/libredis_percentile.so
```

## Commands

### PERCENTILE.MERGE

```PERCENTILE.MERGE <key> values...```

response: len(values)

### PERCENTILE.MERGESORTED

```PERCENTILE.MERGESORTED <key> values...```

response: len(values)


### PERCENTILE.GET

 ```PERCENTILE.GET KEY percentile```

 response: estimated value of percentile