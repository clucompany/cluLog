
#[macro_use]
extern crate cluLog;

#[derive(Debug, Default)]
pub struct Point(u32, f32);

fn main() {
     init_cluLog!();


     let a = Point::default();
     let b = Point(120, 1.345);

     test(&a, &b);

}


fn test(point: &Point, point_2: &Point) {

     if point.0 == 0 {
          trace!("Unknown Behavior");
     }
     
     if point_2.0 > 100 {
          trace!("Unknown Behavior, {:?}", point_2);
     }

     inf!("Point: {:?} == {:?}", point, point_2);
}