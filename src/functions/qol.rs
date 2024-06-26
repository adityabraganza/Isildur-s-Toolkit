use crate::Command;

pub fn sout(s: &str){
    println!("{}", s);
}

pub fn clear(){
    _ = Command::new("clear").status();
}