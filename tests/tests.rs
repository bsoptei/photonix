#[cfg(test)]
mod tests {
    use photonix_derive::*;
    use photonix::*;
    use std::{
        collections::HashMap,
        fmt::Debug,
    };

    #[derive(Debug, Get, GetRef, Set, Modify, PartialEq)]
    pub struct Employee { pub name: String, pub company: Company }

    #[derive(Debug, Get, GetRef, Set, Modify, PartialEq)]
    pub struct Company { pub name: String, pub address: Address }

    #[derive(Debug, Get, GetRef, Set, Modify, PartialEq)]
    pub struct Address { pub city: String, pub street: Street }

    #[derive(Debug, Get, GetRef, Set, Modify, PartialEq)]
    pub struct Street { pub number: u16, pub name: String }

    zoom_all![Employee => Company => Address];
    zoom_all![Employee => Company => String];
    zoom_all![Employee => Company => Address => String];
    zoom_all![Employee => Company => Address => Street => String];
    zoom_all![Employee => Company => Address => Street => u16];

    #[allow(dead_code)]
    #[derive(Debug, GetOption, Set, Modify, PartialEq)]
    enum User {
        Admin,
        Employee(Employee),
    }

    impl ReverseGet<Employee> for User {
        fn reverse_get(value: Employee) -> Self {
            User::Employee(value)
        }
    }

    zoom![User => Employee => Company => Address => Street => u16];

    #[allow(dead_code)]
    #[derive(Debug, GetOption, Set, Modify, SetOption, ModifyOption, PartialEq)]
    enum Json {
        JNull,
        JStr(String),
        JNum(f64),
        JObj(HashMap<String, Json>),
    }

    impl ReverseGet<String> for Json {
        fn reverse_get(value: String) -> Self {
            Json::JStr(value)
        }
    }

    impl ReverseGet<&str> for Json {
        fn reverse_get(value: &str) -> Self {
            Json::JStr(value.into())
        }
    }

    fn h() -> String {
        String::from("hello")
    }

    fn inc(n: u16) -> u16 {
        n + 1
    }

    fn div_by_five_point_two(n: f64) -> f64 {
        n / 5.2
    }

    fn john() -> Employee {
        Employee {
            name: String::from("john"),
            company: awesome_inc(),
        }
    }

    fn awesome_inc() -> Company {
        Company {
            name: String::from("awesome inc"),
            address: ln_high_street(),
        }
    }

    fn ln_high_street() -> Address {
        Address {
            city: String::from("london"),
            street: high_street_23(),
        }
    }

    fn high_street_23() -> Street {
        Street {
            number: 23,
            name: String::from("high street"),
        }
    }

    #[test]
    fn getters() {
        let johns_name: String = john().get();

        assert_eq!(
            "john",
            johns_name.as_str()
        );

        let johns_company_name: String = john().get_second();

        assert_eq!(
            "awesome inc",
            johns_company_name.as_str()
        );

        assert_eq!(
            "london",
            john().get_third().as_str()
        );

        let street_name: String = john().get_fourth();

        assert_eq!(
            "high street",
            street_name.as_str()
        );

        let john2 = john();

        let johns_name2: &String = john2.get_ref();

        assert_eq!(
            "john",
            johns_name2.as_str()
        );

        let johns_company_name2: &String = john2.get_ref_second();

        assert_eq!(
            "awesome inc",
            johns_company_name2.as_str()
        );

        assert_eq!(
            "london",
            john2.get_ref_third().as_str()
        );

        let street_name2: &String = john2.get_ref_fourth();

        assert_eq!(
            "high street",
            street_name2.as_str()
        );
    }

    #[test]
    fn get_option() {
        assert_eq!(
            Some(john()),
            User::Employee(john()).get_option()
        );

        assert_eq!(
            None,
            User::Admin.get_option()
        );
    }

    #[test]
    fn setters() {
        let john_new_address = User::Employee(john()).set_fifth(42);
        assert_eq!(
            Some(42),
            john_new_address.get_option().map(|employee| employee.get_fourth())
        );

        assert_eq!(
            Json::JNum(0.1),
            Json::JNum(0.0).set(0.1)
        );

        assert_eq!(
            Json::JNull,
            Json::JNull.set(0.1)
        );
    }

    #[test]
    fn set_option() {
        assert_eq!(
            Some(Json::JNum(0.1)),
            Json::JNum(0.0).set_option(0.1)
        );

        assert_eq!(
            None,
            Json::JNull.set_option(0.1)
        );
    }

    #[test]
    fn modifiers() {
        let john_new_address = john().modify_fourth(inc);

        assert_eq!(
            24u16,
            john_new_address.get_fourth()
        );

        assert_eq!(
            Json::JNum(10.0),
            Json::JNum(52.0).modify(div_by_five_point_two)
        );

        assert_eq!(
            Json::JNull,
            Json::JNull.modify(div_by_five_point_two)
        );

        let john_new_address = User::Employee(john()).modify_fifth(|num| num + 19);
        assert_eq!(
            Some(42),
            john_new_address.get_option().map(|employee| employee.get_fourth())
        );
    }

    #[test]
    fn modify_option() {
        assert_eq!(
            Some(Json::JNum(10.0)),
            Json::JNum(52.0).modify_option(div_by_five_point_two)
        );

        assert_eq!(
            None,
            Json::JNull.modify_option(div_by_five_point_two)
        );
    }

    #[test]
    fn reverse_get() {
        let hello = Json::reverse_get(h());

        assert_eq!(
            Json::JStr(h()),
            hello
        );
    }

    // If you set a value, and immediately view the value, you get the value that was set.
    fn lens_law1<Lens, Value>(lens: impl Fn() -> Lens, value: Value)
        where Value: Clone + Debug + PartialEq,
              Lens: GetRef<Value> + Set<Value> {
        assert_eq!(
            value.clone(),
            *lens().set(value).get_ref()
        )
    }

    //  If you set a value to `a` and then immediately set the value to `b`,
    // it's the same as if you'd just set the value to `b`.
    fn lens_law2<Lens, Value>(lens: impl Fn() -> Lens, value1: Value, value2: Value)
        where Value: Clone + Debug + PartialEq,
              Lens: Debug + GetRef<Value> + Set<Value> + PartialEq {
        assert_eq!(
            lens().set(value2.clone()),
            lens().set(value1).set(value2)
        );
    }

    // If you get_ref the value, and then immediately set that value back, the value is unchanged.
    fn lens_law3<Lens, Value>(lens: fn() -> Lens)
        where Lens: GetRef<Value> + Set<Value>,
              Value: Clone + Debug + PartialEq {
        let v: Value = lens().get_ref().to_owned();
        assert_eq!(
            v.clone(),
            *lens().set(v).get_ref()
        );
    }

    #[test]
    fn lens_laws() {
        lens_law1(john, String::from("joe"));
        lens_law2(john, String::from("joe"), String::from("mark"));
        lens_law3::<Employee, String>(john);
    }

    pub fn round_trip<Lens, Value>(lens: Lens) -> bool
        where Lens: ReverseGet<Value> + GetOption<Value> + Copy + PartialEq {
        match lens.get_option() {
            None => true,
            Some(value) => lens == Lens::reverse_get(value)
        }
    }

    pub fn round_trip_2<Lens, Value>(v: Value) -> bool
        where Lens: ReverseGet<Value> + GetOption<Value> + Copy + PartialEq,
              Value: PartialEq + Copy {
        Lens::reverse_get(v).get_option() == Some(v)
    }

    #[derive(Clone, Copy, GetOption, Debug, PartialEq)]
    enum XY {
        X(u8),
        Y(f32),
    }

    impl ReverseGet<u8> for XY {
        fn reverse_get(value: u8) -> Self {
            XY::X(value)
        }
    }

    impl ReverseGet<f32> for XY {
        fn reverse_get(value: f32) -> Self {
            XY::Y(value)
        }
    }

    #[test]
    fn prism_laws() {
        assert!(round_trip::<XY, u8>(XY::X(3)));
        assert!(round_trip_2::<XY, u8>(3));
        assert!(round_trip::<XY, f32>(XY::Y(3.0)));
        assert!(round_trip_2::<XY, f32>(3.0));
    }
}

