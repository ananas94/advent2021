use std::io;
use core::slice::Iter;

#[derive(Debug)]
enum Part{
    Open,
    Number { x: u64 },
    Close,
}


fn read_number() -> Option<Vec<Part>> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let mut list = Vec::new();
    for c in inp.trim().chars() {
        match c {
            '['         => { list.push(Part::Open);  },
            ']'         => { list.push(Part::Close); },
            '0'..='9'   => { list.push(Part::Number{  x: c.to_digit(10).unwrap() as u64 }) },
            ','         => {},
            _           => { panic!("bad inp"); }
        }
    }
    if list.is_empty() { return None; }
    Some(list)
}

enum TreeNode {
    Number {x: u64},
    Node { node : Box<Tree> } 
}

struct Tree {
    left: TreeNode,
    right: TreeNode
}

fn make_tree(it:&mut Iter<Part>) -> Tree {
    let left_part = it.next().expect("...");
    let left = match left_part {
        Part::Open      => TreeNode::Node{ node: Box::new(make_tree(it)) }, 
        Part::Number{x} => TreeNode::Number{ x: *x },
        _         => panic!("khekhe")
    };

    let right_part = it.next().expect("...");

    let right= match right_part {
        Part::Open      => TreeNode::Node{ node: Box::new(make_tree(it)) }, 
        Part::Number{x} => TreeNode::Number{ x: *x },
        _         => panic!("khe!")
    };
 
    let close = it.next().expect("...");
    match close {
        Part::Close => {},
        _ => { panic!("wat");}
    }

    Tree{ left: left,  right: right}
}

fn explode(tn:& mut TreeNode, depth: u32) -> (bool,u64, u64) {
    if let TreeNode::Node{node:t} = tn {
        let l = &t.left;
        let r = &t.right;

        if depth >= 4 {
            if let ( TreeNode::Number{x:a}, TreeNode::Number{x:b} ) = ( l, r) {
                   let (av,ab) = (*a,*b);
                   *tn = TreeNode::Number{x:0};
                   return (true,av,ab);
            } 
        }

        let (exploded, mut to_left, mut to_right): (bool, u64,u64) = explode(& mut t.left, depth + 1);
        if exploded  {
            add_to_right(&mut t.right,&mut to_right);
            return (true, to_left, to_right);
        } else {
            let (exploded2,mut to_left2, to_right2) = explode(&mut t.right, depth +1 );
            if exploded2  {
                add_to_left(&mut t.left,&mut to_left2); 
                to_left = 0;
                to_right = to_right2;   //destructuring assignments are unstable
                return (true, to_left, to_right);
            }
        }
    }
    (false,0,0)
}

fn add_to_right(t: & mut TreeNode, add :&mut u64) {
    let mut l = t;
    loop {
        match l {
            TreeNode::Number{x:a} => { *a = *a + *add; *add = 0; break; },
            TreeNode::Node{node:n} => {  l = &mut n.left;  }
        }
    }
}

fn add_to_left(t: & mut TreeNode, add :&mut u64) {
    let mut r = t;
    loop {
        match r {
            TreeNode::Number{x:a} => { *a = *a + *add; *add = 0; break; },
            TreeNode::Node{node:n} => {  r = & mut n.right;  }
        }
    }
}


fn print_tree(t: &TreeNode) {
    match t {
        TreeNode::Number{x:a} => { print!("{}", a); },
        TreeNode::Node{node:n} => { print!("["); print_tree(&n.left); print!(","); print_tree(&n.right); print!("]"); }
    }
}


fn split(tn: &mut TreeNode, depth:u64) -> (bool, u64) {
    if let TreeNode::Node{node:t} = tn {
        let l = &t.left;
        let r = &t.right;

        if let TreeNode::Number{x:a} = l  {
            if *a>9 {
                let left = TreeNode::Number{x:a/2};
                let right = TreeNode::Number{x:(a+1)/2};
                let tree = Tree{left,right};
                t.left = TreeNode::Node{node: Box::new(tree)};
                return (true,depth+1);
            }
        }
        let (was_split, d) = split(&mut t.left, depth+1);
        if was_split { return (was_split, d) }
        if let TreeNode::Number{x:b}  = r {
            if *b>9 {
                let left = TreeNode::Number{x:b/2};
                let right = TreeNode::Number{x:(b+1)/2};
                let tree = Tree{left,right};
                t.right= TreeNode::Node{node: Box::new(tree)};
                return (true,depth+1);
            }
        }
        return split(&mut t.right, depth+1);        
    }
    (false,0)
}

fn explode_all(t: & mut TreeNode) {
    loop {
        let (exploded,_,_) = explode(t,0);
        if !exploded { break;}  
    }
}

fn split_all(t: &mut TreeNode) {
    loop {
        let (spilted, depth) = split(t,0);
        if spilted {
            explode_all(t);
        }
        if !spilted { break;}
    }
}

fn add_tree_nodes(a: TreeNode, b: TreeNode) -> TreeNode {
    let t = Tree{left:a, right:b};
    let mut tn = TreeNode::Node{node: Box::new(t)};
    explode_all(&mut tn);
    split_all(&mut tn);
    return tn;
}


fn magnitude(tn: &TreeNode) -> u64 {
    match tn {
        TreeNode::Number{x:a} =>  {    return *a;  },
        TreeNode::Node{node:t} => {    return 3*magnitude(&t.left)  + 2*magnitude(&t.right);  }
    }
}

fn main() {
    if let Some(next)=read_number() {
        let mut it = next.iter();
        it.next(); //skip first open
        let tree = make_tree(&mut it);
        let mut sum = TreeNode::Node{ node: Box::new(tree)};
        

        while let Some(next) = read_number() {
            let mut it = next.iter();
            it.next(); //skip first open
            let tree = make_tree(&mut it);
            let mut add = TreeNode::Node{ node: Box::new(tree)};

            sum = add_tree_nodes(sum, add);

            explode_all(&mut sum);
            split_all(&mut sum);

        }
        println!("{}", magnitude(&sum));

    }
}
