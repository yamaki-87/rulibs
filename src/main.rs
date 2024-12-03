use rumbok::{AllArgsConstructor, Builder, Data, Singleton, ToString};

fn main() {
    let mut person1 = Person::new_all(0, "james".into(), 20);
    person1.set_age(1);
    person1.set_name("konn".into());
    println!("{}", person1);

    let f = Food::builder().genre(1).id(1).name("test".into()).build();
    println!("{}", f);

    let item_price = ItemPrice::builder()
        .end_date(Some("test".into()))
        .id(1)
        .item_id(2)
        .build();
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use rumbok::Singleton;

    #[test]
    fn singleton_test() -> Result<(), Box<dyn Error>> {
        #[derive(Singleton)]
        struct ItemStore {
            pub id: u32,
            pub name: String,
        }
        let test_num = 403u32;
        let test_str = "kanada";

        let item_store1 = ItemStore::initialize_instance(test_num, test_str.to_string());
        if let Some(item_store2) = ItemStore::get_instance() {
            let addr1 = item_store1 as *const ItemStore;
            let addr2 = item_store2 as *const ItemStore;

            assert_eq!(addr1, addr2, "Singleton instances have diffrent addr!");

            assert_eq!(item_store1.id, test_num);
            assert_eq!(item_store2.id, test_num);

            assert_eq!(item_store1.name, test_str);
            assert_eq!(item_store2.name, test_str);
        }

        Ok(())
    }
}

#[derive(Data)]
struct Person {
    id: u32,
    name: String,
    age: i32,
}

#[derive(Builder, ToString)]
struct Food {
    id: u32,
    name: String,
    genre: u32,
}

#[derive(Builder)]
struct ItemPrice {
    id: u32,
    item_id: u32,
    end_date: Option<String>,
}
