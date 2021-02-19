// Segment Tree Allocator Implement
use super::Allocator;
use alloc::{vec, vec::Vec};
use bit_field::BitArray;
pub struct SegmentTreeAllocator {
    tree: Vec<u8>,
}
impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        let leaf_count = capacity.next_power_of_two();
        //构造一个完全二叉树
        let mut tree = vec![0u8; leaf_count * 2];
        for index in capacity..leaf_count{
            //将叶子节点中被“next_power_of_two”虚构出来的节点设置为已经占用
            tree.set_bit(index, true);
        }
        for index in (1..leaf_count).rev() {
            //这里维护刚才被设置为true的上层节点状态
            let value = tree.get_bit(2 * index) && tree.get_bit(2 * index + 1) && tree.get_bit(index);
            //println_rename!("new index:{},leftson_bit:{},rightson_bit:{},set_bit:{}",index,tree.get_bit(2 * index) , tree.get_bit(2 * index + 1) ,value);
            tree.set_bit(index, value);
        }
        Self{ tree }
    }
    fn alloc(&mut self) -> Option<usize> {
        let mut index = 1;
        if self.tree.get_bit(index) {
            return None;
        }else{
            while index < self.tree.len()/2{//while条件为index指向中间节点
                if !self.tree.get_bit(index * 2){
                    println_rename!{"alloc from left:{}",index * 2};
                    index *= 2;
                }else if !self.tree.get_bit(index * 2 + 1){
                    println_rename!{"alloc from right:{}",index * 2+1};
                    index = index * 2 + 1;
                }else {
                    panic!("Damaged Segement Tree!");
                }
            }
        }
        self.upload_node(index, true);
        //这里返回是申请到了内存中第几个地址
        return Some(index - self.tree.len()/2);
      }
    fn dealloc(&mut self, index: usize) {
        let node = index + self.tree.len()/2;
        self.upload_node(node, false);
    }
}
impl SegmentTreeAllocator{
    fn upload_node(&mut self, mut index: usize, value: bool){
        self.tree.set_bit(index, value);
        while index > 1 {
            index /= 2;
            let v = self.tree.get_bit(2 * index) && self.tree.get_bit(2 * index + 1);
            println_rename!("upload index:{},leftson_bit:{},rightson_bit:{},set_bit:{}",index,self.tree.get_bit(2 * index) , self.tree.get_bit(2 * index + 1) ,v);
            //如果两个子节点都被占用了，则标记父节点为占用
            self.tree.set_bit(index, v);
        }
    }
}