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

  fn add_child(&mut self, weak_parent: Weak<Part>) {
    let child = Part{
      id: 2,
      parent: weak_parent,
      children: Vec::new(),
    };
    self.children.push(Rc::new(child.clone()));
  }
}

fn main() {
  let mut parent = Part::new(1);
  let rc_parent = Rc::new(parent.clone());
  let weak_parent = Rc::downgrade(&rc_parent);

  &parent.add_child(weak_parent);
  println!("{:?}", parent.children[0].parent.upgrade());
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