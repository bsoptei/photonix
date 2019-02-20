#[cfg(test)]
mod tests {
    use photonix_derive::*;
    use photonix::*;
    use std::{
        collections::HashMap,
        fmt::Debug,
    };

    #[derive(Debug, GetRef, Set, PartialEq)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    fn john() -> Person {
        Person { name: String::from("john"), age: 42 }
    }

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

    fn div_by_five_point_two(n: f64) -> f64 {
        n / 5.2
    }

    #[test]
    fn get_option() {
        let json_pi = || Json::JNum(3.14);
        let v1: Option<f64> = json_pi().get_option();
        let v2: Option<String> = json_pi().get_option();

        assert_eq!(
            Some(3.14),
            v1
        );

        assert_eq!(
            None,
            v2
        );
    }

    #[test]
    fn setters() {
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
        assert_eq!(
            Json::JNum(10.0),
            Json::JNum(52.0).modify(div_by_five_point_two)
        );

        assert_eq!(
            Json::JNull,
            Json::JNull.modify(div_by_five_point_two)
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
        lens_law3::<Person, String>(john);
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

