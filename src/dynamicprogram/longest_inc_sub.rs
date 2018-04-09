pub struct LongestIncSub;

impl LongestIncSub {

    pub fn lis(array: &mut Vec<i32>) -> i32 {


        if 0 == array.len(){

            panic!("array is null")
        }

        let mut memo: Vec<i32> = vec![1; array.len()];
        println!("{:?}", memo);

        let mut result: i32 = memo[0];

        let mut  i = 1;
        while i < array.len() {

            let mut  j = 0;
            while j < i {

                if array[i] > array[j] {

                    if memo[i] < (memo[j] + 1) {

                        memo[i] = memo[j] + 1;
                    }
                }

                j = j + 1;
            }
            i = i + 1;
        }

        println!("{:?}", memo);
        let mut  k = 1;
        while k < array.len() {

            if memo[k] > result  {

                result = memo[k];
            }
            k = k + 1;
        }
        return result;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_lis() {

        let mut array: Vec<i32> = vec![10, 9, 2, 5, 3, 7, 101, 18];
        println!("{?:}",lis(&mut array));
        assert!(lis(&mut array), 4);
    }
}