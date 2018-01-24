use std::io;
use std::mem;

#[derive(Clone)]
struct point { x: f64, y: f64 }

#[derive(Clone)]
struct section { p: point, q: point }

#[derive(Clone)]
struct line
{ a: f64, b: f64, c: f64 }

static eps: f64 = 1e-8;

fn segment(s: &section) -> line
{
    let a = s.p.y - s.q.y;
    let b = s.q.x - s.p.x;
    let c = s.p.x * s.q.y - s.q.x * s.p.y;
    let ort = (a * a + b * b).sqrt();
    return line{ a: a / ort, b: b / ort, c: c / ort};
}

fn distance(a: &point, b: &point) -> f64 
{
    return ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt();
}

fn location(l: f64, m: f64, r: f64) -> bool
{
    return l.min(r) <= m && m <= r.max(l);
}

fn nearest<'t>(a: &'t point, b: &'t point, c: &'t point) -> &'t point
{
    if distance(a, b) < distance(a, c) {
        return b;
    }
    else {
        return c;
    }
}

fn cross(a: &section, b: &section) -> Option<point> 
{
    let l1 = segment(a);
    let l2 = segment(b);
    let denom = l1.a * l2.b - l2.a * l1.b;
    if denom.abs() < eps  {
        if (l1.c - l2.c).abs() < eps {
            if location(b.p.x, a.p.x, b.q.x) 
                && location(b.p.y, a.p.y, b.q.y) {
                return Some(a.p.clone());
            }
            if (a.p.x - a.q.x) * (a.p.x - b.p.x) >= 0.
                && (a.p.y - a.q.y) * (a.p.y - b.p.y) >= 0. {
                return Some(nearest(&a.p, &b.p, &b.q).clone());
            }
        }
    }
    else {
        let x = (l1.b * l2.c - l2.b * l1.c) / denom;
        let y = (l2.a * l1.c - l1.a * l2.c) / denom;
        if (a.p.x - a.q.x) * (a.p.x - x) >= 0.
            && (a.p.y - a.q.y) * (a.p.y - y) >= 0.
            && location(b.p.x, x, b.q.x)
            && location(b.p.y, y, b.q.y) {
            return Some(point{ x: x, y: y });
        }
    }
    return None;
}

fn get_sections() -> Option<section>
{
    let mut buf = String::new();
    let res = io::stdin().read_line(&mut buf);
    match res {
        Ok(n) => {
            if n == 0 {
                return None;
            }                 
            let mut iter = buf.split(' ');
            let mut iter2 = iter.next().unwrap().split(',');
            let mut x1 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            let mut y1 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            iter2 = iter.next().unwrap().split(',');
            let mut x2 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            let mut y2 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
                mem::swap(&mut y1, &mut y2);
            }
            let p = point{ x: x1, y: y1 };
            let q = point{ x: x2, y: y2 };
            return Some(section{ p: p, q: q});
        }
        Err(_) => return None
    }
}

fn main() 
{    
    let ray = get_sections().unwrap();
    let mut ans: Option<point> = None;
    loop {
        match get_sections() {
            Some(seg) => {
                match cross(&ray, &seg) {
                    Some(p) => {
                        match ans {
                            Some(cur) => ans = Some(nearest(&ray.p, &cur, &p).clone()),
                            None => ans = Some(p)
                        }
                    }
                    None => {}
                }
            }
            None => break
        }
    }
    match ans {
        Some(p) => {
            println!("{} {}", p.x, p.y);
        }
        None => {}
    }
}