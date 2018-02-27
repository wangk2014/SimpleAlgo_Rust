mod permutations;

use permutations::all_permutations::all_permutations;

fn main() {

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_permutations() {
        let mut array: Vec<i32> = vec![1, 2_000, 3, 500_000];
        all_permutations.perm(&mut array, 0);
    }
}
