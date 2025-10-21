use std::{ collections::HashSet, hash::Hash };



pub fn remove_duplicates<T>(vec: Vec<T>) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut seen = HashSet::new();
    vec.into_iter().filter(|x| seen.insert(x.clone())).collect()
}
