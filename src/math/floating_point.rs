
pub fn normalise_range<T>(&value : T, &ceil : T, &floor : T) -> T{ // remaps values to [1 .. 0]
    return (value + floor) / (ceil - floor);
}

pub fn map_range<T>(&floor : T, &ceil : T, &newFloor : T, &newCeil : T, value: T) -> T{ // remaps value to [newCeil ... newFloor]
    return lerp(newFloor, newCeil, normalise_range(value, ceil, floor))
}

pub fn lerp<T>(&a : T, &b : T, &t : T) -> T{
    return a + (b - a) * t;
}


pub fn sqrt<T>(&v : T) -> T{

}