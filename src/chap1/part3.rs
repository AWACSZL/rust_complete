

pub mod part3{

    pub struct newte{

        name:String,
        email:u64,
        width:u32,
        heigh:u32,
    }
    
    pub fn newtefn(newte: &newte )->(String,u32){
    
    
        let mut thestr:String=String::from(&newte.name);  //记得是引用！！！ 与参数一致，不要乱改所有权
        thestr.push_str(" have get"); //此时thestr为本函数的变量，有可变的所有权，不是结构体的name
    
        (thestr,newte.width*newte.heigh) //thestr所有权给main函数，但是u32的是引用的
    
    }
    
    
    
    struct withimpl{
        with:u32,
        heigh:u32,
    
    }impl withimpl {
        fn area(&self)->u32{
    
            self.with*self.heigh
    
        }
    }
    
    
    enum  withenum {
        name(String),
        v4(u8,u8,u8,u8),
        v6(u8,u8,u8,u8),
        width(u8),
        heigh(u8),
    
    }
    
    
    struct theenum{
        name:withenum,
        val1:withenum,
        val2:withenum,
    
    }

    

}