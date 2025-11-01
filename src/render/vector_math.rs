pub fn normalize(v: (f32, f32, f32)) -> (f32, f32, f32) {
    let len = len(v);
    if len == 0.0 { return v; }
    return (v.0/len, v.1/len, v.2/len)
}

pub fn scalar_multiple(s: f32, v: (f32, f32, f32)) -> (f32, f32, f32) {
    return (v.0 * s, v.1 * s, v.2 * s);
}

pub fn dist(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    return len(difference(a, b));
}

pub fn difference(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    return (a.0 - b.0, a.1 - b.1, a.2 - b.2);
}

pub fn len(v: (f32, f32, f32)) -> f32 {
    return (v.0*v.0 + v.1*v.1 + v.2*v.2).sqrt();
}

pub fn dist_xz(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    return len_xz(difference(a, b));
}

pub fn len_xz(v: (f32, f32, f32)) -> f32 {
    return (v.0*v.0 + v.2*v.2).sqrt();
}

pub fn cross(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    return (a.1*b.2 - a.2*b.1, a.2*b.0 - a.0*b.2, a.0*b.1 - a.1*b.0)
}

pub fn dot(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    return a.0*b.0 + a.1*b.1 + a.2*b.2
}

pub fn rotate_around_axis(v: (f32, f32, f32), axis: (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let c = angle.cos();
    let s = angle.sin();
    let k = 1.0 - c;

    let (x, y, z) = v;
    let (u, v_, w) = normalize(axis);

    return (
        x*(u*u*k + c)     + y*(u*v_*k - w*s) + z*(u*w*k + v_*s),
        x*(v_*u*k + w*s)  + y*(v_*v_*k + c)  + z*(v_*w*k - u*s),
        x*(w*u*k - v_*s)  + y*(w*v_*k + u*s) + z*(w*w*k + c),
    )
}