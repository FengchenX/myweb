use crate::mod1::mod1_1::hello;

mod prelude;

mod mod1;


fn main(){
    hello();
    println!("{}",strStr(String::from("hello world"), String::from("we")));
}

fn strStr(haystack: String, needle: String) ->i32{
    if needle.len()==0{
       return 0;
    }
    let (mut i,mut j)=(0,0);
    while i<haystack.len()-needle.len()+1 {
        while j < needle.len() {
            if haystack.chars().nth(i+j) != needle.chars().nth(j){
                break;
            }
            j+=1;
        }
        if needle.len() ==j{
            return i as i32;
        }
       i+=1;
    }
    -1
}
