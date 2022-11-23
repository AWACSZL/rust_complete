pub mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("app111111");
        }
        pub fn seat_at_table() {
            println!("app222222");
        }
    }

    mod serving {
        //私有-不是pub的话，该模块mod无法被调用
        pub fn take_order() {}

        pub fn server_order() {
            println!("app333333");
        }

        pub fn take_payment() {}
    }
}
