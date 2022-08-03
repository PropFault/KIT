use std::ops::{Add, Div, Mul, Sub};

pub fn normalise_range<T>(value : T, ceil : T, floor : T) -> T
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Div<T, Output=T>
{ // remaps values to [1 .. 0]
    return (value + floor) / (ceil - floor);
}

pub fn map_range<T>(floor : T, ceil : T, new_floor: T, new_ceil: T, value: T) -> T
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Div<T, Output=T> + Mul<T, Output=T>
{ // remaps value to [new_ceil ... new_floor]
    return lerp(new_floor, new_ceil, normalise_range(value, ceil, floor))
}

pub fn lerp<T>(a : T, b : T, t : T) -> T
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T>
{
    return a + (b - a) * t;
}
