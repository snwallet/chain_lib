mod core;

#[cfg(test)]
mod tests {
    use crate::core::global_chain::BLOCK_CHAIN;
    use crate::core::chain::Chain;


    #[test]
    fn chain_test1(){
        let ref mut chain = (*BLOCK_CHAIN).lock().unwrap();
        println!("{:?}",chain);
    }
}
