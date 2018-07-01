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
  fn new() -> Part {
    Part {
      id: 1,
      parent: Weak::new(),
      children: Vec::new(),
    }
  }

  fn add_child(mut self, mut part: Part) {
    self.children.push(Rc::new(part));
    let rc_parent = &Rc::new(self);
    let weak_parent = Rc::downgrade(&rc_parent);
    part.parent = weak_parent;    
  }
}

fn main() {
  let parent = Part::new();
  parent.add_child(Part::new());
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