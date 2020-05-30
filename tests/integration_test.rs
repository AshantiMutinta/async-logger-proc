use async_log::span;
use async_logger_proc_macro::async_logger;

#[async_logger]
fn check_this_out()
{
    println!("test_this")
}


#[test]
fn test_function()
{
    check_this_out();
}