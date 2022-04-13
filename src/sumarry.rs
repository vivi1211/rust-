// u32类型的整数集合求和，参数类型为&[u32],返回类型为Option<u32> 溢出时返回None
pub fn sum(list:&[u32])->Option<u32>{
    let mut add:u32=0;
    for item in list{
        add+=item;
    }
    //match后面的是option类型
    if add<4294967295{
        Some(add)
    }
    else 
    {
        None
    }

}