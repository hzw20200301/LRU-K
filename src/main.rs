//LRU缓存Rust简单实现
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct Node {
    key: i32, value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key, value,
            prev: None, next: None
        }
    }
}

struct LRUCache {
    //hash缓存，便于查找
    cache: HashMap<i32,Rc<RefCell<Node>>>,
    //伪头尾，便于操作节点时不用关心其前后节点是否存在
    head: Rc<RefCell<Node>>, tail: Rc<RefCell<Node>>,
    //大小<=容量
    size: usize, capacity: usize
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = match capacity < 1 {
            true => 1 as usize,
            false => capacity as usize
        };
        let head = Rc::from(RefCell::from(Node::new(-1,-1)));
        let tail = Rc::new(RefCell::new(Node::new(-2,-2)));
        head.borrow_mut().next = Option::from(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());
        Self {
            cache: HashMap::with_capacity(capacity),
            head, tail,
            size: 0 as usize, capacity,
        }
    }

    ///以key获取value
    fn get(&mut self, key: i32) -> i32 {
        if let Some(node_ref) = self.cache.get(&key) {
            self.move_to_head(node_ref);
            return node_ref.borrow().value;
        }
        -1
    }

    ///插入key-value
    fn put(&mut self, key: i32, value: i32) {
        match self.cache.get(&key) {
            // 如果 key 不存在，创建一个新的节点
            None => {
                //到达容量上限，则先清除最老的数据
                if self.size == self.capacity {
                    self.size -= 1;
                    if let Some(node) = self.remove_tail() {
                        self.cache.remove(&node.borrow().key);
                    }
                }
                self.size += 1;
                let node = Rc::new(RefCell::new(Node::new(key,value)));
                self.cache.insert(key,node.clone());
                self.move_to_head(&node);
            }
            // 存在则移动到头部并改写其值
            Some(node_ref) => {
                node_ref.borrow_mut().value = value;
                self.move_to_head(node_ref);
            }
        }
    }

    fn move_to_head(&self, node: &Rc<RefCell<Node>>){
        Self::remove_node(node);
        self.add_to_head(node);
    }

    ///将节点添加到头部
    fn add_to_head(&self, node: &Rc<RefCell<Node>>) {
        //当前节点的prev指向head
        node.borrow_mut().prev.replace(self.head.clone());
        //原头部节点
        if let Some(head_next_ref) = &self.head.borrow().next {
            //当前节点的next指向head的next，完成自身移动到头部
            node.borrow_mut().next.replace(head_next_ref.clone());
            //原来头节点next的prev指向当前节点
            head_next_ref.borrow_mut().prev.replace(node.clone());
        }
        //原来头节点的next指向将当前节点，完成原来的头部后移为第二位
        self.head.borrow_mut().next.replace(node.clone());
    }

    ///从链表中删除尾节点，并返回该节点
    fn remove_tail(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(node) = &self.tail.borrow().prev {
            let node = Self::remove_node(&node);
            return Some(node.clone());
        }
        None
    }

    ///由于需要同时持有node的可变借用,使用unsafe
    unsafe fn remove_node_unsafe(node: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>>{
        if let Some(prev) = &node.borrow().prev {
            if let Some(next) = &node.borrow().next {
                //将前一节点的后置节点指向当前节点的后置节点
                prev.borrow_mut().next.replace(next.clone());
                //预先克隆并返回克隆值,避免返回当前节点处在尾部时下一步操作将其内部Node改变为操作后的尾部
                let node = node.clone();
                //将后一节点的前置节点指向当前节点的前置节点,当移除最老节点时这个操作对于tail而言改变了尾部节点
                (*next.as_ptr()).prev = Some(prev.clone());//这里的裸指针就是使用usafe的原因(next.borrow_mut()将报错:already borrowed: BorrowMutError)
                //(*node.as_ptr()).prev = None;
                //(*node.as_ptr()).next = None;
                return node;
            }
        }
        node.clone()
    }

    //将节点从链表中移除
    fn remove_node(node: &Rc<RefCell<Node>>)  -> Rc<RefCell<Node>>{
        unsafe {
            return Self::remove_node_unsafe(node);
        }

    }
}

///测试
fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1,11);
    cache.put(2,22);
    println!("get 1 {}  1",cache.get(1));
    cache.put(3,33);
    println!("get 2 {}  -1",cache.get(2));
    cache.put(4,44);
    cache.put(3,333);
    println!("get 1 {}  -1",cache.get(1));
    println!("get 3 {}  3",cache.get(3));
    println!("get 4 {}  4",cache.get(4));
}
/*
get 1 11  1
get 2 -1  -1
get 1 -1  -1
get 3 333  3
get 4 44  4
*/
