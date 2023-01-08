# LRU-2
    浙江大学系统软件开发实践课程期末课程项目：
    用RUST编程语言实现LRU-2算法（Realization of LRU-2 algorithm through RUST programming language）  

## 1、LRU算法简介  
    Least Recently Used(最近最少使用)：
      一种常用的页面置换算法，选择最近最久未使用的页面予以淘汰。
      该算法赋予每个页面一个访问字段，用来记录一个页面自上次被访问以来所经历的时间 t，
      当须淘汰一个页面时，选择现有页面中其 t 值最大的，即最近最少使用的页面予以淘汰。
    
    常见实现：使用一个链表保存缓存数据             
    
    算法流程：
      新数据插入到链表头部；
      每当缓存命中（即缓存数据被访问），则将数据移到链表头部；
      当链表满的时候，将链表尾部的数据丢弃

## 2、LRU-K 算法简介 
    LRU-K时LRU算法的进阶版，K是指最近访问页面的次数，LRU算法其实就是LRU-1，其核心思想就是
    将访问一次就能替代的“1”提升为"K"。
    
    实现需要维护两个队列：历史队列和缓存队列。    
     历史队列：保存每次访问的页面，当页面访问次数达到了k次，该页面出栈，并保存至缓存队列；
     若尚未达到k次则继续保存，直至历史队列满，根据一定的缓存策略(FIFO、LRU、LFU)进行淘汰。
     
     缓存队列：保存已经访问k次的页面，当该队列满了之后，则淘汰最后一个页面，也就是
     第k次访问距离现在最久的那个页面。


## 3、LRU-2 算法实现
### 3.1、设计思路
#### 使用哈希映射和双链接列表的组合：
    需满足下列要求：
    1.如果元素有相同值在在 cache 中，则将相同值移动到 cache 的头部；
    2.如果元素有相同值在 history 中，删除 history 中的相同元素，（若 cache 已满则则将 cache 最后一个元素移除），将相同元素移动到 cache 的头部；
    3.如果元素不在 cache 和 history 中，（如果 history 满了，则将 history 最后一个元素移除）。
* 定义一个结构来表示LRU-2缓存。此结构应具有哈希映射字段、两个双链接列表分别代表 history 和 cache以及两链表的阈值大小。
* 实现LRU-2缓存的插入方法。此方法应将传入值作为键，并将值在两链表中的位置索引作为值插入哈希映射中。
* 实现LRU-2缓存的display方法。两个方法分别实时查看两队列状况。

### 3.2、API设计
```Rust
impl<T> Lru2Cache<T>
where
    T: Eq + std::hash::Hash + Copy + std::fmt::Display,
{
     //构造函数创建一个新的LRU缓存，具有给定的容量和要保留的项的数量
    fn new(history_size: usize, cache_size: usize) -> Self {}
    
    //模拟访问数据并进行处理
    fn insert(&mut self, value: T) {}

    //获取历史队列当前状态
    fn get_history_list(&self) -> Vec<T> {}

    //获取缓存队列当前状态
    fn get_cache_list(&self) -> Vec<T> {}
}
```
### 3.3、结构体设计  
```Rust
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
```
## 4、使用流程  
    本次LRU-2算法将数字作为页面进行访问模拟，使用者可以设定 history_size 和 cache_size 分别作为 history 和 cache 队列的阈值大小，
    此后以空格为间隔输入模拟访问序列。这里用 history_size=2、cache_size=2、访问序列为 1、2、1、2、3、1、4、3、5、6进行模拟实验，
    选用原因为尽可能的将不同队列的情况进行考虑。同时优先删除的数据均位于链表头部。
### 示例如下：（令history_size=2、cache_size=2、访问序列为 1、2、1、2、3、1、4、3、5、6）
编译运行：  

![编译运行](https://github.com/hzw20200301/LRU-K/blob/master/images/编译且运行.png)  

输入变量：  

![输入](https://github.com/hzw20200301/LRU-K/blob/master/images/输入.png)  

初始队列：  
<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/1.png" height="175">  
### 下面为输出结果（左）和理想情况的队列变化情况（右）：
输入第1个数字1：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况1.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/2.png" height="175">  

输入第2个数字2：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况2.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/3.png" height="175">

输入第3个数字1：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况3.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/4.png" height="175">

输入第4个数字2：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况4.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/5.png" height="175">

输入第5个数字3：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况5.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/6.png" height="175">

输入第6个数字1：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况6.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/7.png" height="175">

输入第7个数字4：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况7.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/8.png" height="175">

输入第8个数字3：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况8.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/9.png" height="175">

输入第9个数字5：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况9.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/10.png" height="175">

输入第10个数字6：  

<img src="https://github.com/hzw20200301/LRU-K/blob/master/images/队列状况10.png" height="175">    <img src="https://github.com/hzw20200301/LRU-K/blob/master/images/11.png" height="175">

## 5、注意事项
* 注意：在 Rust 1.49 之前，需要使用 #![feature(linked_list_remove)] 开启 LinkedList 类型的 remove() 方法。而使用该语句似乎需要安装nightly版本，我这里采用的方式为使用rustup overwrite设置当前项目使用的channel ：在命令行输入`rustup override set nightly`  

* 我在本次项目中使用的版本为：`rustc 1.68.0-nightly (388538fc9 2023-01-05)`
