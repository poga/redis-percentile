# redis-percentile

Redis module for efficient percentile estimation of streaming or distributed data with [t-digest](https://medium.com/@mani./t-digest-an-interesting-datastructure-to-estimate-quantiles-accurately-b99a50eaf4f7) algorithm.

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

Merge a list of numbers into `<key>`.

**response**: len(values)

### PERCENTILE.MERGESORTED

```PERCENTILE.MERGESORTED <key> values...```

Merge a sorted list of numbers into `<key>`.

**response**: len(values)


### PERCENTILE.GET

 ```PERCENTILE.GET <key> <percentile>```

 **response**: estimated value of percentile

 ##### example

 ```
 PERCENTILE.GET foo 0.9
 ```