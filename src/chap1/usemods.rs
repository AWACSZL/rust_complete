
//注意此时下面的父模块有两个子模块


pub mod front_of_house {  //模块调用测试

    //子模块
    pub mod hosting {

        //内部方法
        pub fn add_to_waitlist(){println!("demoone111111");}

        pub fn seat_at_table() {}

    }



    pub mod serving {
    
        pub fn take_order() {}

        pub fn server_order() {}

        pub fn take_payment() {}
    }


}
