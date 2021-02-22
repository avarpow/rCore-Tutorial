这是实验指导二结束时，整体代码的样例。

实验二将使用此代码来进行。

#### 2021/2/19

给algorithm加入了print方便debug。

过程比较曲折，不会跨crate引用mod，就直接复制了sbi.rs和console.rs为print.rs。

为了防止在os crate中调用引起的宏重名，将println改名为println_rename

update：之所以需要改名因为会报错如下
```bash
error[E0659]: `println` is ambiguous (`macro_rules` vs non-`macro_rules` from other module)
  --> src/memory/frame/allocator.rs:30:9
   |
30 |         println! {"allocator length:{}",range.into().len()};
   |         ^^^^^^^ ambiguous name
   |
note: `println` could refer to the macro defined here
  --> src/console.rs:63:1
   |
63 | / macro_rules! println {
64 | |     ($fmt: literal $(, $($arg: tt)+)?) => {
65 | |         $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
66 | |     }
67 | | }
   | |_^
note: `println` could also refer to the macro imported here
  --> src/memory/frame/allocator.rs:7:5
   |
7  | use algorithm::*;
   |     ^^^^^^^^^^^^
   = help: use `self::println` to refer to this macro unambiguously
```
然后发现在print.rs中去掉#[macro_export]就可以正常使用了

读懂了
https://zhuanlan.zhihu.com/p/350697474 的线段树内存分配器

把SegmentTreeAllocator写了注释

(如果一个节点为true则该节点极其所有的子节点都被占用)

初始化过程如下
1. 创建一个len=2^k(大于所需管理的内存数量)数量的vec,储存线段树节点，其中索引为len/2以上的节点为叶节点
2. 将2^k多出来的部分设置为已经占用
3. 维护上一步中为维护内部节点

申请内存过程如下
1. 从根节点开始，查找false状态的子节点，直到索引大于len/2
2. 标记索引节点为true,并向上维护线段树。
3. 返回值为index-len/2 ,表示是第几个叶节点

释放内存类比申请内存

#### 2021/2/20



