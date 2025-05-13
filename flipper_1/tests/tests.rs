use flipper::flipper::Flipper;

#[ink::test]
fn default_works() {
    let flipper = Flipper::default();
    assert_eq!(flipper.get(), false);
}

#[ink::test]
fn it_works() {
    let mut flipper = Flipper::new(false);
    assert_eq!(flipper.get(), false);
    flipper.flip();
    assert_eq!(flipper.get(), true);
}

#[cfg(feature = "e2e-tests")]
mod e2e {
    use super::*;
    use flipper::flipper::FlipperRef;
    use ink_e2e::ContractsBackend;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let mut constructor = FlipperRef::default();
        let contract = client
            .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await?;
        let call_builder = contract.call_builder::<Flipper>();

        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), false));

        Ok(())
    }

    #[ink_e2e::test]
    async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let mut constructor = FlipperRef::new(false);
        let contract = client
            .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
            .submit()
            .await?;
        let mut call_builder = contract.call_builder::<Flipper>();

        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), false));

        let flip = call_builder.flip();
        client.call(&ink_e2e::bob(), &flip).submit().await?;

        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), true));

        Ok(())
    }
}
