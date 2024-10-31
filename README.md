# rs-numpy-convert

In [main.rs](src/main.rs) there is a short example of creating a Numpy array
in Python and passing it to a Rust code and back. The key idea is that both
technologies use the same byte interpretation of the arrays, so 
`numpy.ndarray.tobytes` and `numpy.frombuffer` work in exactly the way as
data type casting between `[u8]` and common data types in Rust. We do not 
need any intermediate drivers.
