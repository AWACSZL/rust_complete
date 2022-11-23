
pub mod part5{
    use std::fs::File;
    use std::io;
    use std::io::Read;



    pub fn panic_thing(){


        //注意查看下述的内联函数，返回值为一个类似枚举的类型
        let f=File::open("test1.txt");  

        //对枚举的类型进行判断
        let f =match f {
            
            Ok(file)=>{
                println!("打开成功");
                file
            },
            Err(error)=>{
                println!("输出打印，系统出错：{}",error);
                panic!("出错：{}",error)
            }, 
        
        };

    }


    pub fn unwrapandexpect(){

        //很多时候执行可能出错的不需要match，使用unwrap和expect更好
        //其实下述两者基本一样，个人认为使用expect更好一点（能自定义输出的话）

        // let f1 =File::open("test1.txt").unwrap();

        // let f2 =File::open("test1.txt").expect("open failed");

        //==============================aka=====================


    }




    pub fn usesymbol()->Result<String,io::Error>{

        let mut fk = String::new();

        File::open("test1.txt")?.read_to_string(&mut fk);

        Ok(fk)

    }




}