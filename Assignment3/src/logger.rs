
pub struct ConsoleLogger;

pub trait  Logger {
    fn log(&self, message: &str);
}


impl Logger for ConsoleLogger {
    fn log(&self,message:&str) {
        println!("{}", message);
    }
}
