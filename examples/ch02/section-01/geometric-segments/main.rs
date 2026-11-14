#[derive(Debug, Clone, Copy)]
struct Point { x:f64, y:f64 }
#[derive(Debug, Clone, Copy)]
struct Segment { start:Point, end:Point }
impl Segment {
    fn midpoint(self)->Point {
        Point {x:(self.start.x+self.end.x)/2.0,y:(self.start.y+self.end.y)/2.0}
    }
    fn length(self)->f64 {
        (self.end.x-self.start.x).hypot(self.end.y-self.start.y)
    }
}
struct Rectangle { lower_left:Point,upper_right:Point }
impl Rectangle {
    fn new(lower_left:Point, upper_right:Point)->Self {
        assert!(lower_left.x<=upper_right.x && lower_left.y<=upper_right.y);
        Self{lower_left,upper_right}
    }
    fn area(&self)->f64 {
        (self.upper_right.x-self.lower_left.x)*(self.upper_right.y-self.lower_left.y)
    }
    fn perimeter(&self)->f64 {
        2.0*((self.upper_right.x-self.lower_left.x)+(self.upper_right.y-self.lower_left.y))
    }
}
fn main() {
    let s=Segment{start:Point{x:0.0,y:0.0},end:Point{x:3.0,y:4.0}};
    println!("length {}, midpoint {:?}",s.length(),s.midpoint());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn representation_independent_geometry() {
        let s=Segment{start:Point{x:0.0,y:0.0},end:Point{x:3.0,y:4.0}};
        assert_eq!(s.length(),5.0);
        let p=s.midpoint();
        assert_eq!((p.x,p.y),(1.5,2.0));
        let r=Rectangle::new(Point{x:0.0,y:1.0},Point{x:3.0,y:5.0});
        assert_eq!(r.area(),12.0);
        assert_eq!(r.perimeter(),14.0);
    }
}
