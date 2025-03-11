fn main() {
    learn_21_enum();
}

fn learn_21_enum() {
    enum Level {
        #[allow(dead_code)]
        Reguler,
        #[allow(dead_code)]
        Platinum,
        #[allow(dead_code)]
        Premium
    }

    let level = Level::Reguler;
    match level {
        Level::Reguler => println!("Nilai variabel level adalah Reguler"),
        Level::Platinum => println!("Nilai variabel level adalah Platinum"),
        Level::Premium => println!("Nilai variabel level adalah Premium")
    }

    // enum dengan method
    enum Payment{
        CreditCard(String),
        BankTransfer(String, String),
        EWallet(String, String)
    }

    impl Payment {
        fn pay(&self, amount: u32) {
            match self {
                Payment::CreditCard(number) => {
                    println!("Pembayaran sebesar {} melalui kartu kredit dengan nomor {}", amount, number);
                }
                Payment::BankTransfer(number, bank) => {
                    println!("Pembayaran sebesar {} melalui transfer bank dengan nomor {} dan bank {}", amount, number, bank);
                }
                Payment::EWallet(number, provider) => {
                    println!("Pembayaran sebesar {} melalui e-wallet {} dengan nomor {}", amount,provider, number);
                }
            }
        }
    }

    let test_bayar1 = Payment::CreditCard("1234".to_string());
    let test_bayar2 = Payment::BankTransfer("1234".to_string(), "BCA".to_string());
    let test_bayar3 = Payment::EWallet("1234".to_string(), "OVO".to_string());
    test_bayar1.pay(10000);
    test_bayar2.pay(10000);
    test_bayar3.pay(10000);

}
