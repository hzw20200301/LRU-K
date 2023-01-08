#![feature(linked_list_remove)]
use std::collections::{LinkedList, HashMap};
use std::io;

//定义LRU缓存结构
struct Lru2Cache<T> {
    //历史队列，采用FIFO淘汰策略，采用双向链表实现
    history: LinkedList<T>,
    
    //缓存队列，采用LRU淘汰策略，采用双向链表实现
    cache: LinkedList<T>,
    
    //历史队列的容量，即它能容纳的最大条目数
    history_size: usize,
    
    //缓存队列的容量，即它能容纳的最大条目数
    cache_size: usize,
    
    //哈希映射，存储值到索引的映射
    map: HashMap<T, (usize, usize)>,
}

impl<T> Lru2Cache<T>
where
    T: Eq + std::hash::Hash + Copy + std::fmt::Display,
{
     //构造函数创建一个新的LRU缓存，具有给定的容量和要保留的项的数量
    fn new(history_size: usize, cache_size: usize) -> Self {
        Lru2Cache {
            history: LinkedList::new(),
            cache: LinkedList::new(),
            history_size,
            cache_size,
            map: HashMap::new(), 
        }
    }
    
    //模拟访问数据并进行处理
    fn insert(&mut self, value: T) {
        //判断元素是否已在两队列中
        if let Some((_history_index, _cache_index)) = self.map.get(&value) {
            // 如果元素在 cache 中
            if let Some(cache_index) = self.map.get(&value).and_then(|(_, cache_index)| Some(*cache_index)).filter(|index| *index != usize::max_value()) {
                //更改 cache 队列中移动元素后的元素的map映射 cache 索引值
                for element in self.cache.iter_mut().skip(cache_index + 1) {
                    let (_, cache_map_num) = self.map.get(element).unwrap();

                    self.map.insert(*element, (usize::max_value(), cache_map_num - 1));
                }

                // 将其移动到 cache 的尾部
                self.cache.remove(cache_index);
                self.cache.push_back(value);
                
                //更改移动元素map映射索引值
                self.map.insert(value, (usize::max_value(), self.cache.len() - 1));

            // 如果元素有相同值在 history 中
            }else if let Some(history_index) = self.map.get(&value).and_then(|(history_index, _)| Some(*history_index)).filter(|index| *index != usize::max_value()) { 
                //更改 history 队列中移动元素后的元素的map映射 history 索引值
                for element in self.history.iter_mut().skip(history_index + 1) {
                    let (history_map_num, _) = self.map.get(element).unwrap();
                    self.map.insert(*element, (history_map_num - 1, usize::max_value()));
                }

                // 删除 history 中的相同元素
                self.history.remove(history_index);

                // 如果 cache 已满，则将 cache 最后一个元素移除
                if self.cache.len() == self.cache_size {
                    let value_to_remove = self.cache.pop_front().unwrap();
                    self.map.remove(&value_to_remove);
                }

                // 将相同元素移动到 cache 的尾部
                self.cache.push_back(value);

                //更改移动元素map映射索引值
                self.map.insert(value, (usize::max_value(), self.cache.len() - 1));
            } 
        // 如果元素不在 cache 和 history 中
        } else {
                // 如果 history 满，则将 history 最后一个元素移除
                if self.history.len() == self.history_size {
                    if let Some(value_to_remove) = self.history.pop_front() {
                        self.map.remove(&value_to_remove);
                    }                   
                }

                // 将元素插入到 history 的尾部
                self.history.push_back(value);
                self.map.insert(value, (self.history.len() - 1, usize::max_value()));
        }

        //输出元素在 history 和 cache 队列中的索引
        for (key, (first, second)) in &self.map {
            println!("key: {} history_index: {} cache_index: {}", key, first, second);
        }
    }

    //获取历史队列当前状态
    fn get_history_list(&self) -> Vec<T> {
        println!("此时 History 队列大小：{}", self.history.len());
        self.history.iter().copied().collect()
    }

    //获取缓存队列当前状态
    fn get_cache_list(&self) -> Vec<T> {
        println!("此时 Cache 队列大小：{}", self.cache.len());
        self.cache.iter().copied().collect()
    }
}


fn main() {
    println!("请输入历史队列的阈值大小 : ");

    let mut history_size = String::new();
    io::stdin().read_line(&mut history_size).expect("无法读取输入");

    println!("请输入缓存队列的阈值大小 : ");

    let mut cache_size = String::new();
    io::stdin().read_line(&mut cache_size).expect("无法读取输入");

    let history_size: usize = history_size.trim().parse().expect("无法转换为数字");
    let cache_size: usize = cache_size.trim().parse().expect("无法转换为数字");
    let mut cache = Lru2Cache::new(history_size, cache_size);

    println!("请输入一连串以空格间隔的数字：");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取输入");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let mut i = 1;
    
    println!();

    for number in numbers {
        println!("当输入第{}个数字{}时：", i, number);
        println!("元素在map中的索引值情况为：");
        cache.insert(number);
        i = i + 1;
        println!("历史队列与缓存队列情况如下：");
        println!("History: {:?}", cache.get_history_list());
        println!("Cache: {:?}\n", cache.get_cache_list());
    }
}

