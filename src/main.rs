/*
 * This program demonstrates the enumerability of the set of
 * all n-tuples of natural numbers.
 *
 */

fn f2(mut i: i32) -> (i32, i32) {
    /*  This is the inverse of:
     *  f(n, m) = 2^(n - 1)*(2m - 1)
     *  which assigns each ordered pair to a unique natural number. */
    let mut n: i32 = 1;
    while i % 2 == 0 {
        n += 1;
        i /= 2;
    }
    let m: i32 = (i + 1) / 2;
    (n, m)
}

fn f_n(n: i32, i: i32) -> Vec<i32> {
    /* This function gives the i'th n-tuple according to
     * an isomorphism of n-tuples (for a fixed n) to natural numbers.
     */
    let mut nums: Vec<i32> = vec![i];
    while n > nums.len() as i32 {
        let (a, b) = f2(nums.pop().unwrap());
        nums.push(a);
        nums.push(b);
    }
    nums
}

fn main() {
    for n in 1..30 {
        /* Here, all enumerations of n-tuples for a fixed n
         * are combined into a single enumeration.
         */
        let (a, b) = f2(n);
        println!("{:<3} -> {:?}", n, f_n(a, b));
    }
}
