pub fn minmax<'a, E: PartialOrd>(a: &'a E, b: &'a E) -> (&'a E, &'a E) {
    if a <= b { (a, b) } else { (b, a) }
}