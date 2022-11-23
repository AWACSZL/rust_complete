
mod app1;
mod chap1;

use chap1::*;
use app1::*;
//注意mod和use各自分开且都需要，
//每个需要调用的文件夹内需要有mod.rs(名称不可更改)的文件，作用类似路由
//9-5号的rust基础学习基本结束

fn main() {

    println!("H2345432");

    // usemods::front_of_house::hosting::add_to_waitlist();

    // app1::front_of_house::hosting::add_to_waitlist();

    // guessnum::guessnum::guessnum();
    
    // part4::dummass::thevecfn();

    // part4::dummass::somestring();

    // part4::dummass::strslice();

    // part5::part5::panic_thing();

    // part5::part5::unwrapandexpect();

    // part5::part5::usesymbol();


    let  p = part6::part6::Point{x:5,y:10};
    println!("泛型定义为：{},{}",p.x,p.y);

    let nak2 =part6::part6::nak2(120);

    println!("单独泛型函数定义为：{}",nak2);





}


