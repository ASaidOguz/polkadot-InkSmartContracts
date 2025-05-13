use balance_mapping_2::balance_mapping_2::BalanceMapping2;

#[ink::test]
fn default_works() {
    let balance_mapping_2 = BalanceMapping2::default();
    assert_eq!(balance_mapping_2.get_balance(), None);
}

#[ink::test]
fn it_works() {
    // how to send crypto value in tests ? it seems not possible yet sadly... 
    let  balance_mapping_2 = BalanceMapping2::new();
    assert_eq!(balance_mapping_2.get_balance(), None);  
}