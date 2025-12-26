use paq1_authn_core::apikey::services::password_hasher::PasswordHasher;
use paq1_authn_infra::apikey::services::password_hasher::DefaultPasswordHasher;

#[test]
pub fn should_generate_static_hash() {
    let salt = "whatevereeee";

    let expected_hash = "/bjzZ8HutsOX4FyyT4JPVlH9pp6rxFDFh6ce4xn6v3s";

    let sut = DefaultPasswordHasher::new(salt);

    let pure= "wE8hcXvZ9qmEWCK86Yv4GPJw4/0toC0wSThqCpnYoTA=";
    println!("pure : {:?}", pure);

    let hased_computed = sut.hashed(pure).expect("Should have hashed");
    println!("hashed : {:?}", hased_computed);



    assert_eq!(hased_computed.as_str(), expected_hash);
}

#[test]
pub fn should_generate_random_pwd_hash() {
    let salt = "whateverf";
    let sut = DefaultPasswordHasher::new(salt);

    let pure_1 = sut.generate_pure_random().unwrap();
    let pure_2 = sut.generate_pure_random().unwrap();

    assert_ne!(pure_1, pure_2);
}
