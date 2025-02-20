fn main() {

    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let _v2 = v.clone();
    //here two variables own heap values
    //two pointers to the same content is not allowed in rust

    //rust is very smart in terms of memory access, so it detects a race condition
    //as two variable point to same heap

    println!("{:?}",v);
}
