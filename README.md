这是实验指导二结束时，整体代码的样例。

实验二将使用此代码来进行。

2021/2/19
1.

给algorithm加入了print方便debug。

过程比较曲折，不会跨crate引用mod，就直接复制了sbi.rs和console.rs为print.rs。

为了防止在os crate中调用引起的宏重名，将println改名为println_rename

2.
读懂了
https://zhuanlan.zhihu.com/p/350697474 的线段树内存分配器

把SegmentTreeAllocator写了注释
