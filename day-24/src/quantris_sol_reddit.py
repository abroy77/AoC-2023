import argparse
import collections
import sys

# 
#https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kersplf/?utm_source=share&utm_medium=web2x&context=3

def main(filename):
    lines = read_file(filename)
    task = parse_task(lines)
    answer = solve(*task)

    print(answer)

def read_file(filename):
    with open(filename, "r") as f:
        lines = [line.strip() for line in f]
    return lines

Vec3 = collections.namedtuple("Vec3", "x,y,z", defaults = (0, 0, 0))

def parse_task(lines) -> tuple[list[Vec3], list[Vec3]]:
    pts = []
    vels = []
    for line in lines:
        p, _, v = line.partition("@")
        pts.append(Vec3(*map(int, p.split(","))))
        vels.append(Vec3(*map(int, v.split(","))))
    return (pts, vels)

def solve(pts: list[Vec3], vels: list[Vec3]):
    n = len(pts)
    # get three independent hailstones
    # sort hailstones by ascending magnitude of position vector

    p1, v1 = pts[0], vels[0]
    for i in range(1, n):
        if indep(v1, vels[i]):
            p2, v2 = pts[i], vels[i]
            break
    for j in range(i+1, n):
        if indep(v1, vels[j]) and indep(v2, vels[j]):
            p3, v3 = pts[j], vels[j]
            break

    
    rock, S = find_rock(p1, v1, p2, v2, p3, v3)
    return sum(rock) / S

def find_rock(p1: Vec3, v1: Vec3, p2: Vec3, v2: Vec3, p3: Vec3, v3: Vec3):
    
    a, A = find_plane(p1, v1, p2, v2)
    b, B = find_plane(p1, v1, p3, v3)
    c, C = find_plane(p2, v2, p3, v3)
    # (vector, scalar) is returned by find_rock
    # v_rock . a = A
    # v_rock . b = B
    # v_rock . c = C

    sf = 1
    # downscale to avoid overflow
    a = Vec3(a.x // sf, a.y // sf, a.z // sf)
    b = Vec3(b.x // sf, b.y // sf, b.z // sf)
    c = Vec3(c.x // sf, c.y // sf, c.z // sf)
    A //= sf
    B //= sf
    C //= sf

    basis1 = cross(b,c)
    basis2 = cross(c,a)
    basis3 = cross(a,b)


    w = lin(A, basis1, B, basis2, C, basis3)
    t = dot(a, cross(b, c))
    # given that w is integer, so force it here to avoid carrying through
    # imprecision
    # rest of the computation is integer except the final division
    print(f"w before change {w}")
    w = Vec3(round(w.x / t), round(w.y / t), round(w.z / t))
    print(w)

    w1 = sub(v1, w)
    w2 = sub(v2, w)
    ww = cross(w1, w2)

    E = dot(ww, cross(p2, w2))
    F = dot(ww, cross(p1, w1))
    G = dot(p1, ww)
    S = dot(ww, ww)

    rock = lin(E, w1, -F, w2, G, ww)
    return (rock, S)

def find_plane(p1: Vec3, v1: Vec3, p2: Vec3, v2: Vec3) -> tuple[Vec3, int]:
    # get [(p1 - p2) x (v1 - v2) , (p1 - p2) . (v1 x v2)]
    p12 = sub(p1, p2)
    v12 = sub(v1, v2)
    vv = cross(v1, v2)

    return (cross(p12, v12), dot(p12, vv))

def cross(a: Vec3, b: Vec3) -> Vec3:
    return Vec3(a.y*b.z - a.z*b.y, a.z*b.x - a.x*b.z, a.x*b.y - a.y*b.x)

def dot(a: Vec3, b: Vec3) -> int:
    return a.x*b.x + a.y*b.y + a.z*b.z

def sub(a: Vec3, b: Vec3) -> Vec3:
    return Vec3(a.x-b.x, a.y-b.y, a.z-b.z)

def lin(r: int, a: Vec3, s: int, b: Vec3, t: int, c: Vec3):
    # represent the point of intersection of 3 planes as a linear combination
    x = r*a.x + s*b.x + t*c.x
    y = r*a.y + s*b.y + t*c.y
    z = r*a.z + s*b.z + t*c.z
    return Vec3(x, y, z)

def indep(a: Vec3, b: Vec3) -> bool:
    return any(v != 0 for v in cross(a, b))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("filename", nargs="?", default="input.txt")
    args = parser.parse_args()
    sys.exit(main(args.filename))
