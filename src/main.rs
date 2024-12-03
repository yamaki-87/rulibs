use rumbok::{AllArgsConstructor, Builder, Data, ToString};

fn main() {
    let mut person1 = Person::new(0, "james".into(), 20);
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

    let s = Some(1);
    let d = "sfd";
    println!("{:?}", d);
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
