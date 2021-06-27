/// A simple (but not efficient) sorting function
///
/// # Example
/// ```
/// use machiavelli::sort::sort;
///
/// let unsorted = vec![1,5,3,2,4];
/// let sorted = sort(&unsorted, Box::new(|x: &i8| {-x}));
///
/// assert_eq!(vec![5,4,3,2,1], sorted);
/// ```
pub fn sort<T: Clone, U: Ord+Clone> (a: &Vec<T>, f: Box<dyn Fn(&T) -> U>) -> Vec<T> {
    let mut sorted = Vec::<T>::new();
    let mut sorted_f = Vec::<U>::new();
    let mut a_f = Vec::<U>::new();
    
    for x in a {
        a_f.push((*f)(x));
    }
    
    for j in 0..a.len() {
        let mut inserted = false;
        for i in 0..sorted.len() {
            if a_f[j] <= sorted_f[i] {
                sorted.insert(i, a[j].clone());
                sorted_f.insert(i, a_f[j].clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            sorted.push(a[j].clone());
            sorted_f.push(a_f[j].clone());
        }
    }    
    sorted
}
