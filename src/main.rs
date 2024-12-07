use rumbok::{AllArgsConstructor, Builder, Data, Singleton, ToString};

fn main() {
    let f = Food::builder().genre(1).id(1).name("test".into()).build();
    println!("{}", f);

    let item_price = ItemPrice::builder()
        .end_date(Some("test".into()))
        .id(1)
        .item_id(2)
        .build();
    let mut v = vec![];
    v.push(1);
    let s = v.get_mut(0).unwrap();
    *s = 3;
    println!("{:?}", &v);
}

#[cfg(test)]
mod test {
    use rumbok::*;
    use std::error::Error;

    #[test]
    fn singleton_test() -> Result<(), Box<dyn Error>> {
        mod item {
            use rumbok::Singleton;
            #[derive(Singleton)]
            pub struct ItemStore {
                pub id: u32,
                pub name: String,
            }
        }
        // item::ItemStore::new_allがないことを確認すること

        let test_num = 403u32;
        let test_str = "kanada";

        let item_store1 = item::ItemStore::initialize_instance(test_num, test_str.to_string());
        if let Some(item_store2) = item::ItemStore::get_instance() {
            let addr1 = item_store1 as *const item::ItemStore;
            let addr2 = item_store2 as *const item::ItemStore;

            assert_eq!(addr1, addr2, "Singleton instances have diffrent addr!");

            assert_eq!(item_store1.id, test_num);
            assert_eq!(item_store2.id, test_num);

            assert_eq!(item_store1.name, test_str);
            assert_eq!(item_store2.name, test_str);
        }

        Ok(())
    }

    #[test]
    fn data_test() -> Result<(), Box<dyn Error>> {
        #[derive(Data)]
        struct Person {
            id: u32,
            name: String,
            age: i32,
        }

        let (test_id, test_name, test_age) = (1, "alex", 30);
        let person = Person::new_all(test_id, test_name.into(), test_age);
        assert_eq!(&test_id, person.get_id());
        assert_eq!(test_name, person.get_name());
        assert_eq!(&test_age, person.get_age());

        let (set_test_id, set_test_name, set_test_age) = (3, "muko", 1);
        let mut person = Person::default();
        person.set_id(set_test_id);
        person.set_name(set_test_name.into());
        person.set_age(set_test_age);
        assert_eq!(&set_test_id, person.get_id());
        assert_eq!(set_test_name, person.get_name());
        assert_eq!(&set_test_age, person.get_age());

        let (get_mut_test_id, get_mut_test_name, get_mut_test_age) = (2, "als", 39);
        let mut person = Person::default();
        let id = person.get_mut_id();
        *id = get_mut_test_id;
        let name = person.get_mut_name();
        *name = get_mut_test_name.into();
        let age = person.get_mut_age();
        *age = get_mut_test_age;
        assert_eq!(&get_mut_test_id, person.get_id());
        assert_eq!(get_mut_test_name, person.get_name());
        assert_eq!(&get_mut_test_age, person.get_age());

        Ok(())
    }
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
