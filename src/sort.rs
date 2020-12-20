#[allow(dead_code)]
pub struct Sort;

impl Sort{
    pub fn merge_sort<T: PartialOrd+Copy> (arr: &mut [T]){
        Self::merge_sort_util(arr, 0, arr.len()-1);
    }
    fn merge_sort_util<T: PartialOrd+ Copy>(arr: &mut [T], l: usize, r: usize){
        if l>=r{
            return
        }
        let m : usize= (l+r-1)/2;
        Self::merge_sort_util(arr, l, m);
        Self::merge_sort_util(arr, m+1, r);
        Self::merge_arr(arr, l, m, r);
    } 
    fn merge_arr<T: PartialOrd+Copy> (arr: &mut [T], l: usize, m: usize, r: usize){
        let m1 = m-l+1;
        let m2 = r-m;
        let mut arr1 : Vec<T> = Vec::new();
        let mut arr2 : Vec<T> = Vec::new();
        for i in 0..m1{
            arr1.push(arr[i+l]);
        }
        for j in 0..m2{
            arr2.push(arr[m+j+1]);
        }
        let mut i = 0;
        let mut j = 0;
        let mut k = l;
        while i<m1 && j <m2{
            if arr1[i]<arr2[j]{
                arr[k] = arr1[i];
                i+=1;
            }
            else{
                arr[k] = arr2[j];
                j+=1;
            }
            k+=1;
        }
        while i<m1 {
            arr[k] = arr1[i];
            k+=1;
            i+=1;
        }
        while j<m2 {
            arr[k] = arr2[j];
            k+=1;
            j+=1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_int() {
        let mut arr = vec![3, 2, 1];
        Sort::merge_sort(arr.as_mut());
        assert_eq!(arr[..],[1, 2, 3]); 
    }

    #[test]
    fn test_char(){
        let mut arr = vec!['a', 'd', 'c', 'b'];
        Sort::merge_sort(arr.as_mut());
        assert_eq!(arr[..], ['a', 'b', 'c', 'd']);
    }
}
