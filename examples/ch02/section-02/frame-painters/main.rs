use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vect {
    x: f64,
    y: f64,
}

impl Vect {
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn scale(self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Segment {
    start: Vect,
    end: Vect,
}

#[derive(Clone, Copy, Debug)]
struct Frame {
    origin: Vect,
    edge1: Vect,
    edge2: Vect,
}

fn frame_coord_map(frame: Frame) -> impl Fn(Vect) -> Vect {
    move |v| {
        frame
            .origin
            .add(frame.edge1.scale(v.x))
            .add(frame.edge2.scale(v.y))
    }
}

#[derive(Default)]
struct Canvas {
    lines: Vec<Segment>,
}

impl Canvas {
    fn draw_line(&mut self, start: Vect, end: Vect) {
        self.lines.push(Segment { start, end });
    }
}

type Painter = Rc<dyn Fn(Frame, &mut Canvas)>;

fn segments_to_painter(segments: Vec<Segment>) -> Painter {
    Rc::new(move |frame, canvas| {
        let map = frame_coord_map(frame);
        for seg in &segments {
            canvas.draw_line(map(seg.start), map(seg.end));
        }
    })
}

fn transform_painter(painter: Painter, origin: Vect, corner1: Vect, corner2: Vect) -> Painter {
    Rc::new(move |frame, canvas| {
        let m = frame_coord_map(frame);
        let new_origin = m(origin);
        painter(
            Frame {
                origin: new_origin,
                edge1: m(corner1).sub(new_origin),
                edge2: m(corner2).sub(new_origin),
            },
            canvas,
        );
    })
}

fn identity_painter(painter: Painter) -> Painter {
    painter
}

fn flip_vert(painter: Painter) -> Painter {
    transform_painter(
        painter,
        Vect { x: 0.0, y: 1.0 },
        Vect { x: 1.0, y: 1.0 },
        Vect { x: 0.0, y: 0.0 },
    )
}

fn flip_horiz(painter: Painter) -> Painter {
    transform_painter(
        painter,
        Vect { x: 1.0, y: 0.0 },
        Vect { x: 0.0, y: 0.0 },
        Vect { x: 1.0, y: 1.0 },
    )
}

fn rotate90(painter: Painter) -> Painter {
    transform_painter(
        painter,
        Vect { x: 1.0, y: 0.0 },
        Vect { x: 1.0, y: 1.0 },
        Vect { x: 0.0, y: 0.0 },
    )
}

fn beside(left: Painter, right: Painter) -> Painter {
    let split = Vect { x: 0.5, y: 0.0 };
    let paint_left = transform_painter(
        left,
        Vect { x: 0.0, y: 0.0 },
        split,
        Vect { x: 0.0, y: 1.0 },
    );
    let paint_right = transform_painter(
        right,
        split,
        Vect { x: 1.0, y: 0.0 },
        Vect { x: 0.5, y: 1.0 },
    );
    Rc::new(move |frame, canvas| {
        paint_left(frame, canvas);
        paint_right(frame, canvas);
    })
}

fn below(bottom: Painter, top: Painter) -> Painter {
    let split = Vect { x: 0.0, y: 0.5 };
    let paint_bottom = transform_painter(
        bottom,
        Vect { x: 0.0, y: 0.0 },
        Vect { x: 1.0, y: 0.0 },
        split,
    );
    let paint_top = transform_painter(top, split, Vect { x: 1.0, y: 0.5 }, Vect { x: 0.0, y: 1.0 });
    Rc::new(move |frame, canvas| {
        paint_bottom(frame, canvas);
        paint_top(frame, canvas);
    })
}

fn right_split(painter: Painter, n: usize) -> Painter {
    if n == 0 {
        painter
    } else {
        let smaller = right_split(Rc::clone(&painter), n - 1);
        beside(painter, below(Rc::clone(&smaller), smaller))
    }
}

fn up_split(painter: Painter, n: usize) -> Painter {
    if n == 0 {
        painter
    } else {
        let smaller = up_split(Rc::clone(&painter), n - 1);
        below(painter, beside(Rc::clone(&smaller), smaller))
    }
}

fn corner_split(painter: Painter, n: usize) -> Painter {
    if n == 0 {
        painter
    } else {
        let up = up_split(Rc::clone(&painter), n - 1);
        let right = right_split(Rc::clone(&painter), n - 1);
        let top_left = beside(Rc::clone(&up), up);
        let bottom_right = below(Rc::clone(&right), right);
        let corner = corner_split(Rc::clone(&painter), n - 1);
        beside(below(painter, top_left), below(bottom_right, corner))
    }
}

fn square_of_four(
    tl: fn(Painter) -> Painter,
    tr: fn(Painter) -> Painter,
    bl: fn(Painter) -> Painter,
    br: fn(Painter) -> Painter,
) -> impl Fn(Painter) -> Painter {
    move |painter| {
        let top = beside(tl(Rc::clone(&painter)), tr(Rc::clone(&painter)));
        let bottom = beside(bl(Rc::clone(&painter)), br(painter));
        below(bottom, top)
    }
}

fn square_limit(painter: Painter, n: usize) -> Painter {
    let combine4 = square_of_four(flip_horiz, identity_painter, rotate90, flip_vert);
    combine4(corner_split(painter, n))
}

fn main() {
    let unit_frame = Frame {
        origin: Vect { x: 0.0, y: 0.0 },
        edge1: Vect { x: 1.0, y: 0.0 },
        edge2: Vect { x: 0.0, y: 1.0 },
    };
    let wedge = segments_to_painter(vec![
        Segment {
            start: Vect { x: 0.1, y: 0.1 },
            end: Vect { x: 0.8, y: 0.4 },
        },
        Segment {
            start: Vect { x: 0.8, y: 0.4 },
            end: Vect { x: 0.2, y: 0.9 },
        },
    ]);
    let mut canvas = Canvas::default();
    square_limit(wedge, 2)(unit_frame, &mut canvas);
    println!(
        "square_limit(2) rendered {} vector segments",
        canvas.lines.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_frame() -> Frame {
        Frame {
            origin: Vect { x: 0.0, y: 0.0 },
            edge1: Vect { x: 1.0, y: 0.0 },
            edge2: Vect { x: 0.0, y: 1.0 },
        }
    }

    fn approx_seg(a: Segment, b: Segment) -> bool {
        (a.start.x - b.start.x).abs() < 1e-12
            && (a.start.y - b.start.y).abs() < 1e-12
            && (a.end.x - b.end.x).abs() < 1e-12
            && (a.end.y - b.end.y).abs() < 1e-12
    }

    #[test]
    fn rotate90_four_times_is_identity() {
        let base = segments_to_painter(vec![Segment {
            start: Vect { x: 0.2, y: 0.3 },
            end: Vect { x: 0.7, y: 0.8 },
        }]);
        let r4 = rotate90(rotate90(rotate90(rotate90(Rc::clone(&base)))));
        let (mut c1, mut c2) = (Canvas::default(), Canvas::default());
        base(unit_frame(), &mut c1);
        r4(unit_frame(), &mut c2);
        assert_eq!(c1.lines.len(), c2.lines.len());
        assert!(approx_seg(c1.lines[0], c2.lines[0]));
    }

    #[test]
    fn square_limit_stays_inside_unit_square() {
        let stroke = segments_to_painter(vec![Segment {
            start: Vect { x: 0.1, y: 0.2 },
            end: Vect { x: 0.9, y: 0.8 },
        }]);
        let mut canvas = Canvas::default();
        square_limit(stroke, 2)(unit_frame(), &mut canvas);
        assert!(!canvas.lines.is_empty());
        for seg in &canvas.lines {
            for p in [seg.start, seg.end] {
                assert!((-1e-12..=1.0 + 1e-12).contains(&p.x));
                assert!((-1e-12..=1.0 + 1e-12).contains(&p.y));
            }
        }
    }
}
