fn bubble_sort(arr: &mut [i32]){
    let n = arr.len();

    for _ in 0..n{
        for j in 0..n - 1 {
            if arr[j] > arr[j+1]{
                arr.swap(j, j + 1)
            }
        }
    }
}

fn main() {
    let mut data = [ 5, 1, 4, 2, 8];
    bubble_sort(&mut data)
    pritnln!("{}")
}