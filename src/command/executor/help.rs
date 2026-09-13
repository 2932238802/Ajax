use std::println;

// help 指令
pub fn help()
{
    println!(r#"
        exit
            => 退出游戏

        help
            => 打印指令帮助助手
        
        stop
            => 游戏暂停

        save
            => 保存游戏状态

        status  
            -s
                => 打印自己状态的 (基本) 信息
            -sd
                => 打印自己状态的 (详细) 信息
            -t -<thing id>
                => 打印物品的信息    
    "#)
}