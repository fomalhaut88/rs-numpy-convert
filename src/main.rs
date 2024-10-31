use std::mem::size_of;
use std::slice::from_raw_parts;


fn example_from() {
    // # Python commands
    // arr = np.array([1.0, 2.5, 6.25], dtype=np.float64)
    // block = arr.tobytes()
    // list(map(int, block))  # [0, 0, 0, 0, 0, 0, 240, 63, 
    //                           0, 0, 0, 0, 0, 0, 4, 64, 
    //                           0, 0, 0, 0, 0, 0, 25, 64]

    let block: [u8; 24] = [0, 0, 0, 0, 0, 0, 240, 63, 
                           0, 0, 0, 0, 0, 0, 4, 64, 
                           0, 0, 0, 0, 0, 0, 25, 64];
    let ptr = (&block as *const [u8]) as *const f64;
    let data: &[f64] = unsafe { from_raw_parts(ptr, 3) };  // Access without copy

    println!("{:?}", data);  // [1.0, 2.5, 6.25]
}


fn example_to() {
    let arr = [3.5, -0.75, 5.5];
    let ptr = (&arr as *const [f64]) as *const u8;
    let block: &[u8] = unsafe { from_raw_parts(ptr, 24) };  // Access without copy
    println!("{:?}", block);  // [0, 0, 0, 0, 0, 0, 12, 64, 
                              //  0, 0, 0, 0, 0, 0, 232, 191, 
                              //  0, 0, 0, 0, 0, 0, 22, 64]

    // # Python commands
    // np.frombuffer(
    //      bytes([0, 0, 0, 0, 0, 0, 12, 64, 
    //             0, 0, 0, 0, 0, 0, 232, 191, 
    //             0, 0, 0, 0, 0, 0, 22, 64]), 
    //      dtype=np.float64
    // )  # array([ 3.5 , -0.75,  5.5 ])
}


fn main() {
    example_from();
    example_to();
}
