use std::{collections::{HashSet, HashMap}, cell::RefCell, rc::Rc};

#[derive(PartialEq, Debug, Eq)]
struct Person {
    name: &'static str,
    is_alive: bool,
    children: Vec<Rc<RefCell<Person>>>,
}

impl Person {
    pub fn new(name: &'static str) -> Self {
      Self {
            name,
            is_alive: true,
            children: Vec::new(),
        }
    }
}
trait Succession {
    fn order_of_succession(&self) -> Vec<&str>;
}

#[derive(Debug)]
struct Monarchy {
    king: Rc<RefCell<Person>>,
    persons: HashMap<&'static str, Rc<RefCell<Person>>>,
}

impl Monarchy {
    pub fn new(name: &'static str) -> Self {
        let p = Rc::new(RefCell::new(Person::new(name)));
        Self {
            king: p.clone(),
            persons: HashMap::from([(name, p.clone())]),
        }
    }

    pub fn birth(&mut self, child_name: &'static str, parent_name: &str) {
      if let Some(p) = self.persons.get_mut(parent_name) {
          let child = Rc::new(RefCell::new(Person::new(child_name)));
          p.borrow_mut().children.push(child.clone());
          self.persons.insert(child_name, child.clone());  
      }
    }

    pub fn death(&mut self, name: &str) {
      if let Some(p ) = self.persons.get_mut(name) {
        p.as_ref().borrow_mut().is_alive = false;
        return;
      }
      panic!();
    }

    fn dfs(&self, from: Rc<RefCell<Person>>, result : &mut Vec<&str>) {
      let p = from.borrow_mut();
      if p.is_alive {
        result.push(p.name);
      } 
      for i in  0..p.children.len() {
        self.dfs(p.children[i].clone(), result);
      }
    }
}

impl Succession for Monarchy {
    fn order_of_succession(&self) -> Vec<&str>  {
      let mut order = Vec::<&str>::new();
      self.dfs(self.king.clone(), &mut order);
      order
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn monarchy_test() {
        let mut mon = Monarchy::new("Jake");
        mon.birth("Catherine", "Jake");
        mon.birth("Tom", "Jake");
        mon.birth("Celine", "Jake");
        mon.birth("Peter", "Celine");
        mon.birth("Jane", "Catherine");
        mon.birth("Farah", "Jane");
        mon.birth("Mark", "Catherine");

        assert_eq!(vec!["Jake","Catherine","Jane","Farah","Mark","Tom","Celine","Peter"], mon.order_of_succession());

        mon.death("Jake");
        mon.death("Jane");

        assert_eq!(vec!["Catherine","Farah","Mark","Tom","Celine","Peter"], mon.order_of_succession());
    }
}
