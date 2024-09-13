fn main() {

    let v1 = vec![1,2,3,4,5,6,7,8,9,10];

    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.filter(|x| *x%2==1).map(|x| x*2);

    for i in v1_iter2{
        println!("{}",i);
    }
}
