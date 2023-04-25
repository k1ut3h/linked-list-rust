#[derive(Debug)]
struct LinkedList{
    val: i32,
    next: Option<Box<LinkedList>>,
}

impl LinkedList{
    fn new(val:i32)->LinkedList{
        LinkedList { val, next: None}
    }

    fn append(&mut self, val:i32){
        if let Some(ref mut next) = self.next{
            next.append(val);
        } else {
            self.next = Some(Box::new(LinkedList::new(val)));
        }
    }
    fn print(&self) {
        let mut current = self;
        while let Some(ref next) = current.next {
            println!("{}", current.val);
            current = next;
        }
        println!("{}", current.val);
    }

    fn tail(&self)->&LinkedList{
        let mut current = self;
        while let Some(ref next) = current.next{
            current = next;
        }
        current
    }

    fn head(&self)->&LinkedList{
        self
    }
}

fn main() {
    let mut list = LinkedList::new(3);
    for i in 1..=5{
        list.append(i);
    }
    let head = list.head();
    let tail = list.tail();
    println!("{head:?}");
    println!("{tail:?}");
}
