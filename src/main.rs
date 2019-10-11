extern crate k2_creek;

fn main() {
    env_logger::Builder::from_default_env().init();

    let card_data = k2_creek::fetch_carddata();
    k2_creek::write_carddata(&card_data);
}
