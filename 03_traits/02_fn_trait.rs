/*
10:20
https://www.youtube.com/watch?v=HNCzUA1TFbo&t=357s&ab_channel=ComputerScienceCenter

*/

fn map<T, R, F: FnMut(&T) -> R>(xs: &[T], mut f: F) -> Vec<R> {
    let mut res = Vec::with_capacity(xs.len());
    for x in xs {
        let y = f(x);
        res.push(y);
    }
    return res;
}


fn my_inc(x: &i32) -> i32 {
    return x + 1;
}


fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let res: Vec<i32> = map(&v[1..4],  my_inc);
    println!("{:?}", res);
}

