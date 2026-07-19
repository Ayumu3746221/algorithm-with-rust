fn main() {
    let mut data = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
    println!("Before: {:?}", data);
    bubble_sort(&mut data);
    println!("After:  {:?}", data);
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
