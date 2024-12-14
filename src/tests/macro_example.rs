

#[cfg(test)]
mod tests {
    use log_macro::log_handler;

    #[test]
    fn it_works() {
        main();
    }

    #[log_handler]
    fn main(){

    }
}