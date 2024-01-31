

fn smallest(arr: &mut Vec<i32>, m: usize) -> Vec<i32> {//O(mn) algo 
    //(can be truly O(mn) if i stored the current leargest small number (so i woulndt have to shrink the array))
    let mut result: Vec<i32> = Vec::new();

    let mut i: usize = 0; // loop iterater, bound by constant m
    while i < m && i < arr.len() { //while we are still looking for more smallest numbers
        let mut min_index = 0; // current smallest
        let mut index = 1; //current place in the vector

        while index < arr.len() {//loops over the vector and finds the smallest
            if arr[index] < arr[min_index] {
                min_index = index;
            }
            index += 1;
        }

        result.push(arr.remove(min_index));//removes

        i += 1;
    }

    result
}

//insertion sort5
fn isort(arr: &mut Vec<i32>){
    //temporary swap variable
    for i in 1..arr.len() {// that nice
        let mut j = i;//starts 1 above the sorted portion of the vec
        while j>0 && arr[j]<arr[j-1]{//filters down through vec
            arr.swap(j, j-1); //short hand swap fn
            j-=1;
        }
    }
}   

fn gcd(mut a:i32,mut b:i32)->i32{

    while b!=0{
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
    }

fn main() {
    let mut numbers:Vec<i32> = vec![4, 2, 7, 1, 9, 8, 3, 5, 6, 10];
    
    println!("Before sorting: {:?}", numbers);
    println!("{:?}", smallest(&mut numbers, 5));
    isort(&mut numbers);
    println!("GCD of {} and {} is: {}", 120, 50, gcd(120, 50));
    
    println!("After sorting: {:?}", numbers);

}
