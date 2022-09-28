use mona::artifacts::ArtifactSetName;
use mona::common::StatName;



pub fn iter4(
    set_name: ArtifactSetName,
    main_stats: (StatName, StatName, StatName),
    max: &[usize],
    current: &[usize],
    max_sum: usize,
    current_sum: usize,
    current_index: usize) {

}

fn generate_helper<T: FnMut(&[usize]) -> ()>(sum: usize, max: &[usize], current: usize, path: &mut [usize], post: &[usize], cb: &mut T) {
    // println!("sum = {}, current = {}", sum, current);
    if post[current] < sum {
        return;
    }
    if current == max.len() - 1 {
        path[max.len() - 1] = sum;
        // println!("{:?}", path);
        cb(path);
        return;
    }

    for i in 0..max[current].min(sum) {
        path[current] = i;
        generate_helper(sum - i, max, current + 1, path, post, cb);
    }
}

pub fn generate<T: FnMut(&[usize]) -> ()>(
    sum: usize,
    max: &[usize],
    mut cb: T,
) {
    let count = max.len();
    let post_sum_arr = {
        let mut temp = vec![0; count];
        temp[0] = max[count - 1];
        for i in 1..count {
            temp[i] = temp[i - 1] + max[count - i - 1];
        }
        temp.reverse();
        temp
    };
    println!("{:?}", post_sum_arr);

    let mut pp = vec![0; count];
    generate_helper(sum, max, 0, &mut pp, &post_sum_arr, &mut cb);
}

pub fn main() {
    let max = vec![5; 5];
    let mut count = 0;
    generate(
        8,
        &max[..],
        |result| {
            count += 1;
            // println!("{:?}", result);
        }
    );
    println!("{}", count);
}