struct DoubleRef<'a:'b, 'b, T>{
    r: &'a T,
    s: &'b T
}