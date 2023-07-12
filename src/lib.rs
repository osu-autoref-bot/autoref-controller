pub mod error;
pub mod matches;

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn hi() {
        println!("Hello world!");
    }
}
