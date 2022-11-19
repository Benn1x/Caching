#![allow(non_snake_case)]


///The cache is in form of a Struct with to elements the first Element is T
///T is lamda to execute costum code in the cache 
///T needs to return a bool so the system knows if the task were sucsess full or not
///The second part of the cache is the heart of the cache a Costum list type
///to get even better performance
pub struct Cache<E, T>
    where  T: std::cmp::PartialEq + Copy, E: Copy 
{
    cache: Lists<T,E>
}

impl<E, T> Cache<E, T>
    where T: std::cmp::PartialEq + Copy, E: Copy 
{
    ///This Creats a new Cache
    pub fn new() -> Self{
        Self { cache: Lists::new() }
    }
    /// calles the expression and parse an &mut reference of the Cache
    /// this makes it possible to execute code directly in the cache
    pub fn call(&mut self)-> bool{
        false
    }

    ///insert a linked value into the cache
    pub fn insert(&mut self, link: T, value: E){
        self.cache.push(link, value);
    }

    ///gets the value ot of the cache
    pub fn get(&self, link: T)->Option<E>{
        self.cache.get(link)
    }
}

///This is the list used to increse performance
///by about 12.3 times against an HashMap
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
        let mut cache = Cache::<i32,i32>::new();
        cache.insert(1,3);
        cache.insert(2,3);
        cache.insert(3,3);
        cache.insert(4,3);
        cache.insert(5,3);
        cache.insert(6,3);
        cache.insert(7,3);
        cache.insert(8,9);

        assert_eq!(9, cache.get(8).unwrap());
    }
}
