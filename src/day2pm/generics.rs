pub fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n %2 == 0 {
        even
    } else {
        odd
    }
}

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }
}