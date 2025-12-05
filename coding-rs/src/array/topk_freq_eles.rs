use std::collections::HashMap;
use std::io::Read;

fn topk_freq_eles(eles: Vec<i32>, k: i32) {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for ele in eles {
        let mut v = map.entry(ele).or_insert(0);
        *v += 1;
    }

    println!("map = {:?}", map);
    let mut sorted_vec: Vec<_> = map.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
    let (left, _right) = sorted_vec.split_at(k as usize);
    println!("res = {:?}", left.iter().map(|v| *v.0).collect::<Vec<i32>>());
}

#[cfg(test)]
mod tests {
    use crate::array::topk_freq_eles::topk_freq_eles;

    #[test]
    fn test_topk_freq_eles() {
        topk_freq_eles(vec![1,1,1,2,2,3,3,3,3].to_vec(), 2)
    }
}