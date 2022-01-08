fn main() {
    let input = include_str!("i.txt");
    let wires : Vec<String> = input.lines().map(|x| x.trim().parse().unwrap()).collect();
    let wire_a : Vec<String> = wires[0].split(",").map(|x| x.trim().parse().unwrap()).collect();
    let wire_b : Vec<String> = wires[1].split(",").map(|x| x.trim().parse().unwrap()).collect();
    let p1 : i32 = part1(wire_a, wire_b);
    print!("\nPART 1: {}", p1);
}

#[derive(Clone, Copy)]
struct WireHori {
    start: i32,
    end: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct WireVert {
    start: i32,
    end: i32,
    x: i32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    md: i32,
}

fn pathFinder(wire: Vec<String>) -> (Vec<WireHori>, Vec<WireVert>) {
    let mut hori : Vec<WireHori> = Vec::new();
    let mut vert : Vec<WireVert> = Vec::new();
    let mut xC = 0i32;
    let mut yC = 0i32;
    for i in 0..wire.len() {
        let copy = wire.clone();
        let direction = &(copy[i])[0..1];
        let m : i32 = (&(copy[i])[1..]).trim().parse().unwrap();
        match direction {
            "R" => {
                let h = WireHori {
                    start: xC,
                    end:   xC + m,
                    y:     yC,
                };
                hori.push(h);
                xC += m;
                //hori add init x, + x, y; xC + m
            }
            "L" => {
                let h = WireHori {
                    start: xC - m,
                    end:   xC,
                    y:     yC,
                };
                hori.push(h);
                xC -= m;
                //hori add init x, - x, y; xC - m
            }
            "U" => {
                let v = WireVert {
                    start: yC,
                    end: yC + m,
                    x: xC,
                };
                vert.push(v);
                yC += m;
                //vert, copy R
            }
            "D" => {
                let v = WireVert {
                    start: yC - m,
                    end: yC,
                    x: xC,
                };
                vert.push(v);
                yC -= m;
                //vert, copy L
            }
            _   => print!("ERROR: UNKNOWN DIRECTION"),
        }
    }
    return (hori, vert)
}

fn findIntersections(ah: Vec<WireHori>, av: Vec<WireVert>, bh: Vec<WireHori>, bv: Vec<WireVert>) -> Vec<Point> {
    let mut intersections : Vec<Point> = Vec::new();
    for h in ah.clone() {
        for v in bv.clone() {
            if h.y >= v.start && h.y <= v.end && v.x >= h.start && v.x <= h.end {
                let p = Point {
                    x: v.x,
                    y: h.y,
                    md: v.x.abs() + h.y.abs(),
                };
                intersections.push(p);
            }
        }
    }
    for h in bh.clone() {
        for v in av.clone() {
            if  h.y >= v.start && h.y <= v.end && v.x >= h.start && v.x <= h.end {
                let p = Point {
                    x: v.x,
                    y: h.y,
                    md: v.x.abs() + h.y.abs(),
                };
                intersections.push(p);
            }
        }
    }
    return intersections
}

fn part1(wire_a: Vec<String>, wire_b: Vec<String>) -> i32 {
    let (segments_ah, segments_av) = pathFinder(wire_a);
    let (segments_bh, segments_bv) = pathFinder(wire_b);
    let intersections = findIntersections(segments_ah, segments_av, segments_bh, segments_bv);
    return leastMD(intersections)
}

fn leastMD(intersections: Vec<Point>) -> i32 {
    let mut min : i32 = i32::MAX;
    for is in intersections {
        print!("\n{:?}", is);
        if is.md < min && is.md != 0 {
            min = is.md;
        }
    }
    return min
}
