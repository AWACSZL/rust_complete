

pub mod dummass{

    pub fn thevecfn(){   //集合的一般使用

        let mut nv=vec![1,2,3,4,5];

        nv.push(6);

        let nk1=&nv[3];
        let nk2=nv.get(4);

        println!("the vec is {},{:?}",nk1,nk2);


    }

    pub fn somestring(){

        //第一种
        let mut emptystr = String::new();
        emptystr.push_str("engtest-第一种方式");   //在后续添加字符串
        println!("第一种1：{}",emptystr); 

        emptystr.push('s');  //在后续添加字符
        println!("第一种2：{}",emptystr); 




        //第二种
        let mut vallues="engtest-第二种方式的第一种";
        let mut tostrval=vallues.to_string();
        println!("第二种1：{}",tostrval);

        let orsecond ="engtest-第二种方式的第二种".to_string();
        println!("第二种2：{}",orsecond);




        //第三种
        let fromstr = String::from("engtest-第三种创建字符串的方法");
        println!("第三种：{}",fromstr);


        //更新字符串-拼合-format！宏，不需要变更所有权
        let pinghestr = format!("{}456-123{}",tostrval,orsecond);
        println!("拼合为：{}",pinghestr);


        //+号的拼合--有所有权变更
        let mut plussym = tostrval + &orsecond;
        println!("加号的输出为：{}",plussym);



    }


    pub fn strslice(){
        //字符串切片和索引，索引需要使用切片

        let trytoslice = "中文歌曲龙虎榜";
        let trysec =String::from("香港电台新闻报道");

        // let akasli = &trytoslice[3];  //索引字符串不允许直接使用下标

        let adawer=&trytoslice[3..6];  //3-5，注意需要切片的字符的字节数
        let babwaw = &trysec[0..3]; //0-2，可以允许直接切String

        /*
         * 切片字节数 UTF-8 中文简繁体和标点都为3个字节！
         * 
         * 切片的[X..Y]中，切出的片只包含X，不包含Y！
         */



        // println!("索引为：{}",akasli);  //索引字符串不允许直接使用下标
        
        println!("切片索引为：{}",adawer);
        println!("切片索引为：{}",babwaw);



        let aboutchar="this is about char 关于字节的另一种实现";

        for i in aboutchar.chars(){
            print!("{}",i);
        }




    }







}