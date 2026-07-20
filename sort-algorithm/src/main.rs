fn main() {
    let mut data = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
    println!("Before: {:?}", data);
    bubble_sort(&mut data);
    println!("After:  {:?}", data);

    let mut data = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
    println!("Before: {:?}", data);
    insert_sort(&mut data);
    println!("After:  {:?}", data);

    let data = vec![5, 2, 9, 1, 5, 6];
    println!("Before: {:?}", data);
    let result = merge_sort(data);
    println!("After:  {:?}", result);
}

fn bubble_sort<T: Ord>(data: &mut Vec<T>) {
    let len = data.len();
    let mut end = len;
    while end > 0 {
        for i in 0..data.len() - 1 {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
            }
        }
        end -= 1;
    }
}

fn insert_sort<T: Ord>(data: &mut Vec<T>) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn merge_sort<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    let len = v.len();
    if len <= 1 {
        return v;
    }
    let right = v.split_off(len / 2);
    let left = merge_sort(v);
    let right = merge_sort(right);
    merge(left, right)
}

fn merge<T: Ord>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut l = left.into_iter().peekable();
    let mut r = right.into_iter().peekable();
    loop {
        match (l.peek(), r.peek()) {
            (Some(a), Some(b)) => {
                if a <= b {
                    result.push(l.next().unwrap());
                } else {
                    result.push(r.next().unwrap());
                }
            }
            (Some(_), None) => {
                result.extend(l);
                break;
            }
            (None, Some(_)) => {
                result.extend(r);
                break;
            }
            (None, None) => break,
        }
    }
    result
}
