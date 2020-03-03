// Universal Array Utils


pub struct Array_Utils;


impl Array_Utils
{
    pub fn get_difference<T>(arr1: &Vec<T>, arr2: &Vec<T>) -> Vec<T>
    where T: std::cmp::PartialEq + Copy + Clone
    {
        let mut cur_vector = Vec::new();

        for item in arr2
        {
            if Self::index_of(arr1, item) == -1
            {
                cur_vector.push(*item);
            }
        }

        return cur_vector;
    }

    pub fn index_of<T>(arr: &Vec<T>, value: &T) -> isize
    where T: std::cmp::PartialEq + Copy + Clone
    {
        let mut step = 0;

        for item in arr
        {
            if item == value
            {
                return step;
            }

            step+=1;
        }

        return -1;
    }

    pub fn filter<T>(arr: &Vec<T>, iter: &dyn Fn(&&T) -> bool ) -> Vec<T>
    where T: std::cmp::PartialEq + Copy + Clone
    {
        let iter =  arr.into_iter().filter(iter).map(|&x| x);

        return iter.collect::<Vec<T>>();
    }

    pub fn map<T, Q>(arr: &Vec<T>, iter: &dyn Fn(T) -> Q ) -> Vec<Q>
    where T: Clone
    {
        let mut cur_arr = Vec::new();

        for elem in arr
        {
            cur_arr.push(iter(elem.clone()));
        }
        
        return cur_arr;
    }
    
    pub fn max<T>(arr: &Vec<T>) -> T
    where T: std::cmp::PartialEq + Copy + Clone + std::cmp::Ord
    {
        arr.clone().sort();
    
        return arr[arr.len() - 1];
    }
}