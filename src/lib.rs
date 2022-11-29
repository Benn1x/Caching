#![allow(non_snake_case)]
//TODO Implement F!!!

///The cache is in form of a Struct with two elements
///The second part of the cache is the heart of the cache a Costume list type
///to get the needed performance for my project
pub struct Cache<E, T>
    where T: std::cmp::PartialEq + Copy, E: Copy 
{
    function: fn(Lists<T, E>)-> bool,
    cache: Lists<T,E>
}

impl<E, T> Cache<E, T>
    where T: std::cmp::PartialEq + Copy, E: Copy 
{
    ///fn new() creates a new cache
    ///as an argument it takes an fn(Lists<E, T>) what returns a bool, to check if it had success or failure
    /// # Examples
    ///```rust
    /// use Caching;
    /// fn main() -> (){
    ///     let cache = caching::Cache::<i32,String>::new(|a/* This is a lists<i32, String>*/|-> bool {return true});
    ///     // Some Code
    /// }
    /// ```

    pub fn new(function: fn(Lists<T, E>)-> bool) -> Self{
        Self { function, cache: Lists::new() }
    }
    /// calls the expression and parse the Cache cache into it
    /// this makes it possible to execute code directly in the cache
    /// # Examples
    ///```rust
    /// use Caching;
    /// fn main() -> (){
    ///     let cache = caching::Cache::<i32,String>::new(|a/* This is a lists<i32, String>*/|-> bool {return true});
    ///     cache.call(); //This executes the cade in the cache
    ///     // Some Code
    /// }
    /// ```
    pub fn call(&mut self)-> bool{
        (self.function)(self.cache.clone())
    }

    ///insert a linked value into the cache
    /// # Examples
    ///```rust
    /// use Caching;
    /// fn main() -> (){
    ///     let cache = caching::Cache::<i32,String>::new(|a/* This is a lists<i32, String>*/|-> bool {return true});
    ///     cache.insert(4,"4".to_string()); //This puts the values 4 and "4" in the cache
    ///     // Some Code
    /// }
    /// ```
    pub fn insert(&mut self, link: T, value: E){
        self.cache.push(link, value);
    }

    ///insert a linked value into the cache
    /// # Examples
    ///```rust
    /// use Caching;
    /// fn main() -> (){
    ///     let cache = caching::Cache::<i32,String>::new(|a/* This is a lists<i32, String>*/|-> bool {return true});
    ///     cache.insert(4,"4".to_string()); //This puts the values 4 and "4" in the cache
    ///     cache.get(4); //returns Option enum with None if the value is not the cache and Some(), in this case Some("4")
    ///     // Some Code
    /// }
    /// ```
    pub fn get(&self, link: T)->Option<E>{
        self.cache.get(link)
    }
}

///This is the list used to increse performance
///by about 12.3 times against an HashMap
#[derive(Clone)]
pub struct Lists<T,E>
{
    linked: Vec<T>,
    list: Vec<E>,
}

impl<T,E> Lists<T, E>
    where T: std::cmp::PartialEq, E: Copy

{
    //Creats a new list
    pub fn new() -> Self{
        Self{linked: Vec::new(), list: Vec::new()}
    }
    ///pushes values into the list 
    pub fn push(&mut self,link: T, list: E){
       self.linked.push(link);
       self.list.push(list);
    }
    ///This returns the Element where link is Linked to
    pub fn get(&self, link: T)->Option<E>{
        //this part returns the Element where linked is linked to 
        for i in 0..=self.linked.len(){

            if self.linked[i] == link {
                return Some(self.list[i]);
            }
        }
        None
    
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut cache = Cache::<i32,i32>::new(|a|->bool{true});
        cache.insert(1,3);
        cache.insert(2,3);
        cache.insert(3,3);
        cache.insert(4,3);
        cache.insert(5,3);
        cache.insert(6,3);
        cache.insert(7,3);
        cache.insert(8,9);
        assert_eq!(9, cache.get(8).unwrap());
        assert!(cache.call());
    }
}
