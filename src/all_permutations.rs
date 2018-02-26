pub struct all_permutations;

impl all_permutations {

    pub fn perm(&self, array: &mut Vec<i32>, start: usize) {
        if start == (array.len() - 1) {
            println!("{:?}", array);
        }

        let mut i = start;
        while i < array.len() {
            array.swap(i, start);
            all_permutations.perm(array, start + 1);
            array.swap(i, start);
            i = i + 1;
        }
    }
}


//#[cfg(test)]
//mod tests {
//
//    use super::*;
//
//    #[test]
//    fn test_permutations() {
//        let mut array: Vec<i32> = vec![1, 2_000, 3, 500_000];
//        all_permutations.perm(&mut array, 0);
//    }
//}