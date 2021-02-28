
/// 字义微信结果类型
pub type WeChatResult<T> = Result<T, WeChatError>;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
