

pub mod part6{

    pub struct maintest <T,U>{

        X:T,
        Y:U,    
    }


    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Point<T> {

        pub fn x(&self) -> &T {
            &self.x
        }

    }

    //注意作用域和引用：传进来的参数是引用主函数的变量，返回值是引用本函数的变量
    //因此两个都在输出后一直存在

    //*或者从一开始主函数变量就不要进行引用，直接入值赋予该函数所有权
    pub fn nak2<T>(na: T)->T{

        let mut ka =  na;
        ka

    }
    
    

}