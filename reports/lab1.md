## 实现的功能
为TaskControlBlock添加计数各系统调用的CountSyscall结构体  
在syscall函数添加调用是增加对应系统调用的次数  
实现syscall trace  
## 简答
1. 使用rustsbi 0.4  
依次返回
> PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.  
> IllegalInstruction in application, kernel killed it.  
> IllegalInstruction in application, kernel killed it.

在U态使用S态的特权指令后发生Exception::StoreFault和Exception::IllegalInstruction异常  
被内核的tarp处理程序捕获后打印出来  
2. 
    1. 指向了内核栈上分配好的taskcontext.(1)task间切换(2)系统调用后还原  
    2. 处理了sstatus sepc sscratch寄存器,还原用户态的运行环境,配置进入用户态的pc和栈指针
    3. x2和x4分别是堆栈指针和线程指针.内核堆栈指针在内核运行中不变,线程指针不需要
    4. sp中的值变成了用户的堆栈指针,sscratch变成内核的堆栈指针
    5. sret  因为堆栈指针已经变为用户程序的堆栈指针,执行完sret后pc也将切换到用户程序
    6. sp变为内核堆栈指针 srccratch变为用户程序的指针
    7. 使用call后切换


**荣誉准则**
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

>《rCore-Tutorial-Guide-2025S 文档》《RISC-V特权模式文档》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。
   我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。
   我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。
   我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。
   我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。