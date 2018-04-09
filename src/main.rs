//mod permutations;
//use permutations::all_permutations::AllPermutations;

//mod traversal;

pub fn perm(array: &mut Vec<i32>, start: usize) {

    if start == 10 {
//        println!("{:?}", array);
    }

    let mut i = start;
    while i < array.len() {
        array.swap(i, start);
        perm(array, start + 1);
        array.swap(i, start);
        i = i + 1;
    }
}



fn main() {
    let mut array: Vec<i32> = vec![10, 9, 2, 5, 3, 7, 101, 18];


//    println!("{}",lis(&mut array));

}
//
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
