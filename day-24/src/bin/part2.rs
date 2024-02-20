use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, character::complete, sequence::tuple, IResult};
use std::env;
use std::fs;




type Vec3D = [i128; 3];


fn downscale(v: Vec3D, factor: i128) -> Vec3D {
    [v[0] / factor, v[1] / factor, v[2] / factor]

}

fn cross(a: Vec3D, b: Vec3D) -> Vec3D {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot(a: Vec3D, b: Vec3D) -> i128 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn triple_cross(a: Vec3D, b: Vec3D, c: Vec3D) -> i128 {
    dot(a, cross(b, c))
}

fn add(a: Vec3D, b: Vec3D) -> Vec3D {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: Vec3D, b: Vec3D) -> Vec3D {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

struct Hailstone {
    position: [i128;3],
    velocity: [i128;3],
}

fn independent(a: &Vec3D, b: &Vec3D) -> bool {
    let cross_p = cross(*a, *b);
    cross_p.iter().any(|x| *x != 0) 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, hailstones) = parse_input(&contents).unwrap();

    // sort em
    // sort_stones(&mut hailstones);
    let solution = get_solution(hailstones);
    println!("Solution: {:?}", solution);
}


fn get_plane(ha: &Hailstone, hb: &Hailstone) -> (Vec3D, i128) {
    let p12 = sub(ha.position, hb.position);
    let v12  = sub(ha.velocity, hb.velocity);
    let vv = cross(ha.velocity, hb.velocity);

    return(cross(p12,v12), dot(p12, vv))

}

fn get_solution(hailstones: Vec<Hailstone>) -> usize {
    // for 3 different combinations of pairs of hailstones get A and c
    // and concatenate

    // get 3 independent linear systems
    let h1 = &hailstones[0];
    let mut h2 = &hailstones[1];
    let mut h3 = &hailstones[2];
    // get h2
    for i in 1..hailstones.len() {
        if independent(&h1.velocity, &hailstones[i].velocity) {
            h2 = &hailstones[i];
            break;
        }
    }
    // get h3
    for j in 0..hailstones.len() {
        if independent(&h1.velocity, &hailstones[j].velocity) && independent(&h2.velocity, &hailstones[j].velocity) {
            h3 = &hailstones[j];
            break;
        }

    }

    // we now have 3 hailstones with linearly independent velocity vectors

    // let's get the planes of each pair of hailstones
    let (va, sa) = get_plane(h1, h2);
    let (vb, sb) = get_plane(h1, h3);
    let (vc, sc) = get_plane(h2, h3);

    // numbers are too big for i128. Let's downscale
    let sf: i128 = 1000_00;

    let va = downscale(va, sf);
    let vb = downscale(vb, sf);
    let vc = downscale(vc, sf);

    // downscale the scalars
    let sa = sa / sf;
    let sb = sb / sf;
    let sc = sc / sf;

    let basis1 = cross(vb, vc);
    let basis2 = cross(vc, va);
    let basis3 = cross(va, vb);

    let unscaled_intersection = three_plane_intersection(
        sa, basis1, sb, basis2, sc, basis3);
    println!("unscaled_intersection: {:?}", unscaled_intersection);
    let normalisation_factor = triple_cross(va, vb,vc);
    // w is stone velocity
    let w = [unscaled_intersection[0] / normalisation_factor, unscaled_intersection[1] / normalisation_factor, unscaled_intersection[2] / normalisation_factor];
    println!("w: {:?}", w);

    // now we need to find the intersection point
    // given w
    let w1 = sub(h1.velocity, w);
    let w2 = sub(h2.velocity, w);

    // find the intersection of the 
    // lines : h1.position + t * w1
    // and h2.position + s * w2
    

    // use algebra to find the intersection
    let t = (w2[0] * (h1.position[1] - h2.position[1]) - w2[1] * (h1.position[0] - h2.position[0])) / (w1[0] * w2[1] - w1[1] * w2[0]);
    let _s = (h1.position[0] - h2.position[0] + t * w1[0]) / w2[0];

    let intersection = add(h1.position, [t * w1[0], t * w1[1], t * w1[2]]);

    // sum the intersection coordinates
    let ans = intersection.iter().map(|x| x.abs()).sum::<i128>();
    return ans as usize;

}

fn three_plane_intersection(
    r: i128,
    a: Vec3D,
    s: i128,
    b: Vec3D,
    t: i128,
    c: Vec3D,
) -> Vec3D
{
    let x = r * a[0] + s * b[0] + t * c[0];
    let y = r * a[1] + s * b[1] + t * c[1];
    let z = r * a[2] + s * b[2] + t * c[2];
    [x, y, z]

}

fn parse_position(input: &str) -> IResult<&str, (i128, i128, i128)> {
    let (input, (p0, _, p1, _, p2)) = tuple((
        complete::i128,
        tag(", "),
        complete::i128,
        tag(", "),
        complete::i128,
    ))(input)?;

    Ok((input, (p0 as i128, p1 as i128, p2 as i128)))
}

fn parse_velocity(input: &str) -> IResult<&str, (i128, i128, i128)> {
    let (input, (v0, _, v1, _, v2)) = tuple((
        complete::i128,
        tag(", "),
        complete::i128,
        tag(", "),
        complete::i128,
    ))(input)?;

    Ok((input, (v0 as i128, v1 as i128, v2 as i128)))
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let (input, (position, velocity)) =
        separated_pair(parse_position, tag(" @ "), parse_velocity)(input)?;
    let position = [position.0, position.1, position.2];
    let velocity = [velocity.0, velocity.1, velocity.2];

    Ok((input, Hailstone { position, velocity }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hailstone>> {
    separated_list1(newline, parse_hailstone)(input)
}
