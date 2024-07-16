fn main() {
    
        // let name ="Mawia";
        // let num :i8=10;
        // let num11 :i8=10;
        // let num2 :i16=18;
        // let num11 :u16=19;
        // let num3 :i32=9;
         //let num4 :u32=5;
         //let num5 :i64=45;
         //let num6 :u64=13;
         //let num7 :i128=10;
         //let num8 :u128=15;
         //let num9 :isize=-10;
         //let num10 :usize=10;
         //let number: f32 = 110.56;
         //let number: f64 = 110.45;
         //character
         //let character:char='t';
         //let character2:char='🎂';
         //let text_1 : &str = "Welcome to programming";
         //let text_2: String = String:: from("Hello there..."); // Define TEXT2 as a constant
         //let text2: String = "Heloooo".to_string(); // Define a new variable text2
         //println!("hello, {}!", name);
         //println!("{}",num);
         //println!("{}",num2);
         //println!("{}",num4);
         //println!("{}",num5);
         //println!("{}",num7);
         //println!("{}",num8);
         //println!("{}",num10);
         //println!("{}",number);
         //println!("{}",character);
         //println!("{}",character2);
          
        // println!("{}",text_1);
        // println!("Hello...{}",text_2);
         
        //let result = sum(5, 10);
        // println!("Result: {}", result);
         
         
    // }
     //Functions
    // #[allow(dead_code)]
    // fn sum(num_1: i32,num_2: i32){
        // let  num_3: i32 = num_1+num_2;
         //println!("the sum of num_one and num_two{}",num_3);
    // }
     //fn sum2(num_1: i32,num_2: i32)-> i32{
        // let  num_3: i32 = num_1+num_2;
        // println!("the sum of num_one and num_two{}",num_3);
        // return num_3;
    // }
    // fn sum3(num_1: i32,num_2: i32)-> i32{
        // let  num_3: i32 = num_1+num_2;
        // println!("the sum of num_one and num_two{}",num_3);
        // num_3
    // }

    //const NUMBER_1: i32 = 101;
    //let mut number_1_string: String = NUMBER_1.to_string();
    //number_1_string.push_str("People in the room");
    //println!("Hey {}", number_1_string);

    //let input_1 = "21";
    //let _input_number: i32 = input_1.parse().expect("was expecting an integer");
    //. parse tells the compiler  input as a parameter

    //println!("hey {}", _input_number);
    //casting
    //let input_6: i16 = 200;
    //let _input_7: i32 =input_6 as i32;
    //println!("hey{_input_7}");
// SLiCiNG
     //let (_data,_status,_reason)=do_sum_maths(45,22);
     //println!("Sum={_data} was it success{_status}reason: {_reason}");


//ENUMS
    //const AGE: i32 =13;
    //#[derive(Debug)]
    //enum STATUS {
        //ADULT, CHILD
    //}
    //let user_status: STATUS =if AGE >18{
        //STATUS::ADULT
    //}else{
        //STATUS::CHILD
    //};
    //println!("The status is{:?}",user_status);

    //let a: i32 = 5;
    //let b: i32 = 5;
    //if a > b {
        //println!("a is greater than b");
   // } else if a == b {
       // println!("a is equal to b");
   // }
    //if a < b {
        //println!("a is less than b");
    //}
    
    //let mut counter: i32 = 0;

    //loop{
        //counter = counter + 1;
        //println!("I am at number {}", counter); // Changed {counter} to {}
 
        //if counter == 1 {
            //println!("Starting point");
//}
 
        //if counter == 10 {
           // break;
        // }
    //lecture 5
        
 }
   

    //let mut _array=[1,2,3,4,5,6,7,8,9];
    //let mut index:usize=5;
    //while index !=0 {
            //println!("looping through indes{index}items in");
            //index -=1;

        
    //}
        //println!("LIFTOFF!!!");
        //let mut num: i32 = 3;
        //while num != 0 {
            //println!("{}", num);
            //num -= 1;
            //println!("the result is  {}", num);

        //println!("LIFTOFF!!!");
         // }
    

//fn do_sum_maths(_par_1: i32 ,_par_2: i32)->(i32, bool,String){
   // return(par_1+par_2,true,"success".to_string())
    //return (0,false,"failed for some reason ".to_string())
//}
