#[macro_use]
mod macro_test;

use std::rc::{Rc, Weak};
use std::error::Error;

#[derive(Debug, Clone)]
struct Part {
  id: usize,
  parent: Weak<Part>,
  children: Vec<Rc<Part>>,
}

impl Part {
  fn new(id: usize) -> Part {
    Part {
      id: id,
      parent: Weak::new(),
      children: Vec::new(),
    }
  }

  fn add_child(&mut self, part: &mut Part, weak_parent: Weak<Part>) {
    self.children.push(Rc::new(part.clone()));
    // let rc_parent = Rc::new(self.clone());
    // let weak_parent = Rc::downgrade(&rc_parent);
    // println!("{:?}", weak_parent.upgrade());

    part.parent = weak_parent;
    println!("in {:?} id: {}", part.parent.upgrade(), part.id);
  }
}

fn main() {
  let mut parent = Part::new(1);
  let mut child = Part::new(2);
  let rc_parent = Rc::new(parent.clone());
  let weak_parent = Rc::downgrade(&rc_parent);

  &parent.add_child(&mut child, weak_parent);

  // println!("{:?}", weak_parent.upgrade());
  // println!("{:?}", parent.children[0].parent.upgrade());
  println!("{:?}", child.parent.upgrade());
  // println!("{:?}", child);
  // println!("{:?}", parent.children[0].id);
}

/* succeed but it's completely difference what i want */
// let parent = Part {
//   id: 1,
//   parent: Weak::new(),
//   children: Vec::new(),
// };
// let rc_parent = Rc::new(parent);
// let mut parent_copy = rc_parent.clone();

// let child = Part {
//   id: 2,
//   parent: Rc::downgrade(&rc_parent),
//   children: Vec::new(),
// };
// let rc_child = Rc::new(child);
// Rc::make_mut(&mut parent_copy).children.push(rc_child.clone());

// println!("{:?}", rc_child.parent.upgrade());