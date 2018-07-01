#[macro_use]
mod macro_test;

use std::rc::{Rc, Weak};
use std::error::Error;

#[derive(Debug)]
struct Part {
  id: usize,
  parent: Weak<Part>,
  children: Vec<Rc<Part>>,
}

impl Part {
  // fn new() -> Result<Rc<Part>, Error> {
  fn new() -> Result<Part, Rc<Part>> {

    let mut parent = Part {
      id: 1,
      parent: Weak::new(),
      children: Vec::new(),
    };

    let mut child = Part {
      id: 2,
      parent: Weak::new(),
      children: Vec::new(),
    };
    let rc_child = Rc::new(child);
    parent.children.push(rc_child.clone());
    child.parent = Rc::downgrade(&Rc::new(parent));

    Rc::try_unwrap(rc_child)
  }
}

fn main() {
  let child = Part::new().unwrap();
  let parent = child.parent.upgrade();

  println!("{:?}", child);
    // println!("{}", i32 == i32);
}



    // .get().unwrap()
    // child
      // let mut rc_child = parent.children.get_mut(0).unwrap();
      // let mut child = Rc::get_mut(rc_child).unwrap();
      // child.parent = Rc::downgrade(&Rc::new(parent));
    
    // child.parent = Rc::downgrade(&Rc::new(parent));
    // parent: Rc::downgrade(&Rc::new(parent)),
    // parent