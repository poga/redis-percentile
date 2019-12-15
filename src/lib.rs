#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::{parse_float, Context, NextArg, RedisError, RedisResult};
use std::os::raw::c_void;
use tdigest::TDigest;

struct MyType {
    data: TDigest,
}

static MY_REDIS_TYPE: RedisType = RedisType::new(
    "mytype123",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: Some(free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,
    },
);

unsafe extern "C" fn free(value: *mut c_void) {
    Box::from_raw(value as *mut MyType);
}

fn alloc_merge_unsorted(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let key = args.clone().into_iter().skip(1).next_string()?;

    let nums = args
        .into_iter()
        .skip(2)
        .map(|s| parse_float(&s))
        .collect::<Result<Vec<f64>, RedisError>>()?;

    let len = nums.len();

    let key = ctx.open_key_writable(&key);

    match key.get_value::<MyType>(&MY_REDIS_TYPE)? {
        Some(value) => {
            let new_data = value.data.merge_unsorted(nums);
            value.data = new_data
            // key.set_value(&MY_REDIS_TYPE, value)?;
        }
        None => {
            let mut data = TDigest::new_with_size(10);
            data = data.merge_unsorted(nums);
            key.set_value(&MY_REDIS_TYPE, MyType { data })?;
        }
    }

    Ok(len.into())
}

fn alloc_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let key = ctx.open_key(&key);

    let value = match key.get_value::<MyType>(&MY_REDIS_TYPE)? {
        Some(value) => value.data.estimate_quantile(0.9).to_string().into(),
        None => ().into(),
    };

    Ok(value)
}

//////////////////////////////////////////////////////

redis_module! {
    name: "alloc",
    version: 1,
    data_types: [
        MY_REDIS_TYPE,
    ],
    commands: [
        ["tdigest.merge", alloc_merge_unsorted, "write"],
        ["tdigest.get", alloc_get, "readonly"],
    ],
}
