fn main() {
    /*let   name:String=String::from("        Hello .       ");
    println!("Before : {}",name);
    let name=function(&name);
    println!("After : {}",name);*/
    let test1="We need more spaces.";
    assert_eq!(function(test1),"We need more spaces.");
    let test2="     There's a space in front.";
    assert_eq!(function(test2),"There's a space in front.");
    let test3="     There's a space to the rear.      ";
    assert_eq!(function(test3),"There's a space to the rear.");
    let test4="     We are surrounded by space!       ";
    assert_eq!(function(test4),"We are surrounded by space!");
    let test5="       ";
    assert_eq!(function(test5),"");
    let test6="";
    assert_eq!(function(test6),"");    
    let test7=" \u{2661} ";
    assert_eq!(function(test7),"\u{2661}");
    println!("Test Passed!");

}
fn function(var:&str)->&str{
    let mut first=0;
    let mut last=0;
    let mut flag=false;
    let bytes=var.as_bytes();
   // println!("{:?}",bytes);
    for (index,&item) in bytes.iter().enumerate()
    {
        if item != b' ' && flag != true
        {
            //println!("in first {} {}",flag,item as char);
            flag=true;
            first=index ;
            //println!("in first {}",flag);
        }
        else if flag==true && item!=b' '
        { 
            last = index+1;
        }
    }
   return &var[first ..last];
}