use rumbok::{AllArgsConstructor, Builder, Data, Singleton, ToString};

fn main() {
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
    use rumbok::*;
    use std::{error::Error, path::PathBuf};

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
            path: Option<PathBuf>,
        }

        let (test_id, test_name, test_age, test_path) =
            (1, "alex", 30, PathBuf::from(r"C:\program"));
        let person = Person::new_all(test_id, test_name.into(), test_age, Some(test_path.clone()));
        assert_eq!(&test_id, person.get_id());
        assert_eq!(test_name, person.get_name());
        assert_eq!(&test_age, person.get_age());
        assert_eq!(&test_path, person.get_path().as_ref().unwrap());

        let (set_test_id, set_test_name, set_test_age, set_test_path) =
            (3, "muko", 1, PathBuf::from("/usr/local"));
        let mut person = Person::default();
        person.set_id(set_test_id);
        person.set_name(set_test_name.into());
        person.set_age(set_test_age);
        person.set_path(Some(set_test_path.clone()));
        assert_eq!(&set_test_id, person.get_id());
        assert_eq!(set_test_name, person.get_name());
        assert_eq!(&set_test_age, person.get_age());
        assert_eq!(&set_test_path, person.get_path().as_ref().unwrap());

        let (get_mut_test_id, get_mut_test_name, get_mut_test_age, get_mut_test_path) =
            (2, "als", 39, PathBuf::from(r"C:\windows"));
        let mut person = Person::default();
        let id = person.get_mut_id();
        *id = get_mut_test_id;
        let name = person.get_mut_name();
        *name = get_mut_test_name.into();
        let age = person.get_mut_age();
        *age = get_mut_test_age;
        let path = person.get_mut_path();
        *path = Some(get_mut_test_path.clone());
        assert_eq!(&get_mut_test_id, person.get_id());
        assert_eq!(get_mut_test_name, person.get_name());
        assert_eq!(&get_mut_test_age, person.get_age());
        assert_eq!(&get_mut_test_path, person.get_path().as_ref().unwrap());

        assert_eq!(
            "Person { id: 2 name: \"als\" age: 39 path: Some(\"C:\\\\windows\") }",
            format!("{}", person)
        );

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
