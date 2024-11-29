// https://www.codewars.com/kata/5592e3bd57b64d00f3000047
pub fn find_nb(m: u64) -> i32 {
    // clever
    // let mut sum = 0_u64;
    // let l = (0_u64..)
    //     .take_while(|&x| {
    //         sum += x.pow(3);
    //         sum < m
    //     })
    //     .count() as i32;
    // if sum == m {
    //     l
    // } else {
    //     -1
    // }

    if m < 1 {
        return -1;
    }

    let mut sum = m;
    let mut count: i32 = 0;

    while sum > 0 {
        count += 1;
        if let Some(new_sum) = sum.checked_sub((count as u64).pow(3)) {
            sum = new_sum;
        } else {
            return -1;
        }
    }

    if sum == 0 {
        count
    } else {
        -1
    }
}
