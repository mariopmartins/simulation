use pathfinding::prelude::{absdiff, astar};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
  }

  fn successors(&self) -> Vec<(Pos, u32)> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
         .into_iter().map(|p| (p, 1)).collect()
  }
}

#[test]
fn test() {
 static GOAL: Pos = Pos(4, 6);
 let result = astar(&Pos(1, 1), |p| p.successors(), |p| p.distance(&GOAL) / 3, |p| *p == GOAL);
 println!("--> {:?}", result);
 assert_eq!(result.expect("no path found").1, 4);
}

