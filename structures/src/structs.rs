struct Point {
        x: f64,
        y: f64,
}

struct Line {
        start: Point,
        end: Point,
}

pub fn structures() {
        let p = Point { x: 3.0, y: 4.0 };
        println!("point p is at ({},{})", p.x, p.y);

        let p2 = Point { x: 5.0, y: 10.0 };
        let my_line = Line { start: p, end: p2 };
        println!(
                "myLine starts in ({},{}) and end in ({},{})",
                my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y
        );
}
