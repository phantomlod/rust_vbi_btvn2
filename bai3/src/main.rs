fn main() {
    let mut values = vec![10, 11, 12];
    let v = &mut values;
    //let u = v.into_iter();
    let mut max = 0;
    
    //println!("into iter v la {:?}",v.into_iter());
    //test into_iter()
    for i in v.into_iter() {
        println!("{}", i);
    }
    //for n in &mut values {
    for n in v.into_iter() {
        max = std::cmp::max(max, *n);
    }

    // println!("max is {}", max);
    // println!("Converting to percentages of maximum value...");
    //for n in &mut values {
    for n in v {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}