fn add<T: std::ops::Add<Output=T> >(a: T, b: T) -> T {
    a+b
}

struct Point<T> { x: T, y: T }

enum Result<T,E> {
    Ok(T),
    Err(E),
}
