#[cfg(test)]
mod tests {
    use super::super::{TripartiteConfig,get_tripartite_config,Component};
    
    use wechat_redis::{RedisConfig,get_redis_conf};
    #[test]
      fn test_redis() {
        let tripart_config: TripartiteConfig = get_tripartite_config();
        let redis_config: RedisConfig = get_redis_conf();
        let comp = Component::new(tripart_config.clone(), redis_config.clone());
        let x=actix_rt::System::new().block_on(comp.fetch_authorizer_info("wx999317f16de96ce3"));
        println!("{:?}",rs);
    }
}