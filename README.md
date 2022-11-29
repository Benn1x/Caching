# Caching
how do you create a Cache?
```rust
use Caching;
fn main()->{
    // in this example we have an i32 where an String is linked to
    let cache = caching::Cache::<i32,String>::new(|a/* This is a lists<i32, String>*/|-> bool {return true});
    //                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //                                              This is a lambda stored in the cache
    //it can be executed this way:
    cache.call();
    //to store values in the cache call:
    cache.insert(4,"4");
    // to get the value what is linked to 4 call:
    let example = cache.get(4);//returns Option enum with None if the value is not the cache and Some(), in this case Some("4")
}
```
Any questions open a Issue on [Github](https://github.com/Benn1x/Caching/issues)
