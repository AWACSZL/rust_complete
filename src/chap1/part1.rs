

pub mod chapt1{

    pub fn datatype(){

        const THECONST1: u32 =60*60*3;
        let cannot:u64=456789;
        let mut cancha:u32=0123456;
    
        println!("the three num is {},{},{}",THECONST1,cannot,cancha);
    
    }
    
    
    pub fn valuefunc(mut theuniv:u32,thestrv:char)->u32{  //包括返回的函数值定义
    
        if theuniv<456789{
            theuniv=147852;
    
        }else if theuniv==123456 {
            theuniv=369852;
    
        }else{
            theuniv=789456;
    
        }
    
        println!("the value is {},{}",theuniv,thestrv);
    
        theuniv   //注意这个是表达式 所以后面没有分号，可以理解为返回值不能带分号
    
    }
    

}