pub struct AllPermutations;

impl AllPermutations {

    pub fn perm(&self, array: &mut Vec<i32>, start: usize) {
        if start == (array.len() - 1) {
            println!("{:?}", array);
        }

        let mut i = start;
        while i < array.len() {
            array.swap(i, start);
            AllPermutations.perm(array, start + 1);
            array.swap(i, start);
            i = i + 1;
        }
    }
}
