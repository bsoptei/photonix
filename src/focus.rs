/// Returns the target field of a container (struct) by value, consumes the container (unless it implements `Copy`).
///
/// Auto-derive creates the implementation for all fields, works for structs (both with named and unnamed fields) if
/// - all fields have concrete types,
/// - all field types are different.
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(Get)]
/// pub struct Person {
///     pub name: String,
///     pub age: u8,
/// }
///
/// let joe = || Person { name: String::from("Joe"), age: 42 };
///
/// assert_eq!(42u8, joe().get());
///
/// let joe_name: String = joe().get();
///
/// assert_eq!("Joe", joe_name.as_str());
///
///```
///
pub trait Get<Value> {
    fn get(self) -> Value;
}

/// Returns the target field of a container (struct) by reference, keeps the container.
///
/// Auto-derive creates the implementation for all fields, works for structs (both with named and unnamed fields) if
/// - all fields have concrete types,
/// - all field types are different.
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(GetRef)]
/// pub struct Person {
///     pub name: String,
///     pub age: u8,
/// }
///
/// let joe = Person { name: String::from("Joe"), age: 42 };
///
/// assert_eq!(42u8, *joe.get_ref());
///
/// let name: &String = joe.get_ref();
///
/// assert_eq!("Joe", name.as_str());
///
///```
///
pub trait GetRef<Value> {
    fn get_ref(&self) -> &Value;
}

/// Updates the field of a container (struct/enum) with the provided value. Consumes the original container (unless it implements `Copy`), returns the updated one.
///
/// Auto-derive creates the implementation for all fields in structs and enum variants, works for structs and enums (both with named and unnamed fields) if
/// - all fields have concrete types,
/// - all field types are different.
///
///If you are using the derive macro, and the actual enum variant happens to be different from the target variant, it returns the original version (see example).
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(Set)]
/// pub struct Person {
///     pub name: String,
///     pub age: u8,
/// }
///
/// let joe = Person { name: String::from("Joe"), age: 42 };
/// let joe_older = joe.set(43u8); // help the compiler by specifying the field type
///
/// assert_eq!(43, joe_older.age);
///
/// let joel = joe_older.set(String::from("Joel"));
///
/// assert_eq!("Joel", joel.name.as_str());
///
/// #[derive(Debug, Set, PartialEq)]
/// pub enum Deviation {
///     Signed(i32),
///     Unsigned(u32),
/// }
///
/// let d = Deviation::Signed(-10);
/// let d2 = Deviation::Unsigned(10);
///
/// let d_new = d.set(10i32);
///
/// assert_eq!(Deviation::Signed(10), d_new);
///
/// let d2_new = d2.set(10i32);
///
/// assert_eq!(Deviation::Unsigned(10), d2_new);
///```
///
pub trait Set<Value> {
    fn set(self, new_value: Value) -> Self;
}

/// Updates the field of a container (struct/enum) by applying the provided function on the target value. Consumes the original container (unless it implements `Copy`), returns the updated one.
///
/// Auto-derive creates the implementation for all fields in structs and enum variants, works for structs and enums (both with named and unnamed fields) if
/// - all fields have concrete types,
/// - all field types are different.
///
/// If you are using the derive macro, and the actual enum variant happens to be different from the target variant, it returns the original version (see example).
///
/// # Examples
///
///```
/// # use photonix::*;
/// #[derive(Modify)]
/// pub struct Person {
///     pub name: String,
///     pub age: u8,
/// }
///
/// let joe = Person { name: String::from("Joe"), age: 42 };
/// let joe_older = joe.modify(|n: u8| n + 1); // help the compiler by specifying the field type
///
/// assert_eq!(43, joe_older.age);
///
/// let joel = joe_older.modify(|name: String| name + "l");
///
/// assert_eq!("Joel", joel.name.as_str());
///
/// #[derive(Debug, Modify, PartialEq)]
/// pub enum Deviation {
///     Signed(i32),
///     Unsigned(u32),
/// }
///
/// let d = Deviation::Signed(-10);
/// let d2 = Deviation::Unsigned(10);
///
/// let d_neg = d.modify(|n: i32| -n);
///
/// assert_eq!(Deviation::Signed(10), d_neg);
///
/// let d2_neg = d2.modify(|n: i32| -n);
///
/// assert_eq!(Deviation::Unsigned(10), d2_neg);
///```
///
pub trait Modify<Value> {
    fn modify(self, f: impl FnOnce(Value) -> Value) -> Self;
}

/// Returns the target field of a container (enum) by value as an `Option`, consumes the container (unless it implements `Copy`).
///
/// Auto-derive creates the implementation for all fields in enum variants, works for enums (both with named and unnamed fields) if
/// - all fields have concrete types,
/// - all field types are different.
///
/// If you're using the derive macro, and the actual enum variant is different from the target variant, it returns `None` (see example).
///
/// # Examples
///
///```
/// # use photonix::*;
/// #[derive(Debug, GetOption, PartialEq)]
/// pub enum Deviation {
///     Signed(i32),
///     Unsigned(u32),
/// }
///
/// let d = || Deviation::Signed(-10);
///
/// let signed_value: Option<i32> = d().get_option();
///
/// assert_eq!(Some(-10), signed_value);
///
/// let unsigned_value: Option<u32> = d().get_option();
///
/// assert_eq!(None, unsigned_value);
///
/// let d2 = Deviation::Unsigned(10);
///
/// let unsigned_value2: Option<u32> = d2.get_option();
///
/// assert_eq!(Some(10), unsigned_value2);
///```
///
pub trait GetOption<Value> {
    fn get_option(self) -> Option<Value>;
}

/// Updates the field of a container (enum) with the provided value. Consumes the original container (unless it implements `Copy`), returns the updated one wrapped in an `Option`.
///
/// Auto-derive creates the implementation for all fields in enum variants, works for enums (both with named and unnamed fields) if
/// - all fields have concrete types,
/// - all field types are different.
///
/// If you are using the derive macro, and the actual enum variant happens to be different from the target variant, it returns `None` (see example).
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(Debug, SetOption, PartialEq)]
/// pub enum Deviation {
///     Signed(i32),
///     Unsigned(u32),
/// }
///
/// let d = Deviation::Signed(-10);
/// let d2 = Deviation::Unsigned(10);
///
/// let d_new = d.set_option(10i32);
///
/// assert_eq!(Some(Deviation::Signed(10)), d_new);
///
/// let d2_new = d2.set_option(10i32);
///
/// assert_eq!(None, d2_new);
///```
///
pub trait SetOption<Value>
    where Self: Sized {
    fn set_option(self, new_value: Value) -> Option<Self>;
}

/// Updates the field of a container (enum) by applying the provided function on the target value. Consumes the original container (unless it implements `Copy`), returns the updated one wrapped in an `Option`.
///
/// Auto-derive creates the implementation for all fields in enum variants, works for enums (both with named and unnamed fields) if
/// - all fields have concrete types,
/// - all field types are different.
///
/// If you are using the derive macro, and the actual enum variant happens to be different from the target variant, it returns `None` (see example).
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(Debug, ModifyOption, PartialEq)]
/// pub enum Deviation {
///     Signed(i32),
///     Unsigned(u32),
/// }
///
/// let d = Deviation::Signed(-10);
/// let d2 = Deviation::Unsigned(10);
///
/// let d_neg = d.modify_option(|n: i32| -n);
///
/// assert_eq!(Some(Deviation::Signed(10)), d_neg);
///
/// let d2_neg = d2.modify_option(|n: i32| -n);
///
/// assert_eq!(None, d2_neg);
///```
pub trait ModifyOption<Value>
    where Self: Sized {
    fn modify_option(self, f: impl FnOnce(Value) -> Value) -> Option<Self>;
}

/// Constructs the relevant enum variant based on the input.
///
/// This trait does not have a corresponding auto-derive macro.
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(Debug, PartialEq)]
/// pub enum Deviation {
///     Signed(i32),
///     Unsigned(u32),
/// }
///
/// impl ReverseGet<i32> for Deviation {
///     fn reverse_get(value: i32) -> Self {
///         Deviation::Signed(value)
///     }
/// }
///
/// impl ReverseGet<u32> for Deviation {
///     fn reverse_get(value: u32) -> Self {
///         Deviation::Unsigned(value)
///     }
/// }
///
/// assert_eq!(Deviation::Signed(3), Deviation::reverse_get(3i32));
/// assert_eq!(Deviation::Unsigned(3), Deviation::reverse_get(3u32));
///
///```
pub trait ReverseGet<Value> {
    fn reverse_get(value: Value) -> Self;
}

pub mod composites {
    use super::*;

    /// A variant of [`Get`], reaching two levels deep in the data structure.
    ///
    /// [`Get`]: ../trait.Get.html
    ///
    /// # Examples
    ///```
    /// # use photonix::*;
    /// #[derive(Get)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(Get)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// pub struct Address(String);
    ///
    /// //             Level 1   Level 2    Parent type
    /// //               |          |           |
    /// impl GetSecond<Company, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let company_name = john_doe.get_second();
    ///
    /// assert_eq!("Acme Corporation", company_name.as_str());
    ///
    ///```
    pub trait GetSecond<LevelOne, LevelTwo>
        where
            LevelOne: Get<LevelTwo>,
            Self: Get<LevelOne> + Sized {
        fn get_second(self) -> LevelTwo {
            self.get().get()
        }
    }

    /// A variant of [`GetRef`], reaching two levels deep in the data structure.
    ///
    /// [`GetRef`]: ../trait.GetRef.html
    ///
    /// # Examples
    ///```
    /// # use photonix::*;
    /// #[derive(GetRef)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(GetRef)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// pub struct Address(String);
    ///
    /// //                      Level 1     Level 2     Parent type
    /// //                          |          |            |
    /// impl<'a> GetRefSecond<'a, Company, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let company_name = john_doe.get_ref_second();
    ///
    /// assert_eq!("Acme Corporation", company_name.as_str());
    ///
    ///```
    pub trait GetRefSecond<'a, LevelOne, LevelTwo>
        where
            LevelOne: GetRef<LevelTwo> + 'a,
            Self: GetRef<LevelOne> + 'a {
        fn get_ref_second(&'a self) -> &'a LevelTwo {
            self.get_ref().get_ref()
        }
    }

    /// A variant of [`Set`], reaching two levels deep in the data structure.
    ///
    /// [`Set`]: ../trait.Set.html
    ///
    /// # Examples
    ///```
    /// # use photonix::*;
    /// #[derive(Get, Modify)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(Get, Set)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// pub struct Address(String);
    ///
    /// //            Level 1    Level 2     Parent type
    /// //                |        |           |
    /// impl SetSecond<Company, String> for Employee {}
    /// impl GetSecond<Company, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let john_doe_new = john_doe.set_second(String::from("Evil Corp"));
    ///
    /// assert_eq!("Evil Corp", john_doe_new.get_second().as_str());
    ///
    ///```
    pub trait SetSecond<LevelOne, LevelTwo>
        where
            LevelOne: Set<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn set_second(self, new_value: LevelTwo) -> Self {
            self.modify(|level_one| level_one.set(new_value))
        }
    }

    /// A variant of [`Modify`], reaching two levels deep in the data structure.
    ///
    /// [`Modify`]: ../trait.Modify.html
    ///
    /// # Examples
    ///```
    /// # use photonix::*;
    /// #[derive(Get, Modify)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(Get, Modify)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// pub struct Address(String);
    ///
    /// //            Level 1    Level 2     Parent type
    /// //                |        |           |
    /// impl ModifySecond<Company, String> for Employee {}
    /// impl GetSecond<Company, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let john_doe_new = john_doe.modify_second(|name| name.replace("Corporation", "Inc"));
    ///
    /// assert_eq!("Acme Inc", john_doe_new.get_second().as_str());
    ///
    ///```
    pub trait ModifySecond<LevelOne, LevelTwo>
        where
            LevelOne: Modify<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn modify_second(self, f: impl FnOnce(LevelTwo) -> LevelTwo) -> Self {
            self.modify(|level_one| level_one.modify(f))
        }
    }

    /// A variant of [`Get`], reaching three levels deep in the data structure.
    ///
    /// [`Get`]: ../trait.Get.html
    ///
    /// # Examples
    ///```
    /// # use photonix::*;
    /// #[derive(Get)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(Get)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// #[derive(Get)]
    /// pub struct Address(String);
    ///
    /// //            Level 1    Level 2  Level 3   Parent type
    /// //                |        |        |           |
    /// impl GetThird<Company, Address, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let company_address = john_doe.get_third();
    ///
    /// assert_eq!("4 Foo Road, Bar City", company_address.as_str());
    ///
    ///```
    pub trait GetThird<LevelOne, LevelTwo, LevelThree>
        where
            LevelTwo: Get<LevelThree>,
            LevelOne: Get<LevelTwo>,
            Self: Get<LevelOne> + Sized {
        fn get_third(self) -> LevelThree {
            self.get().get().get()
        }
    }

    /// A variant of [`GetRef`], reaching three levels deep in the data structure.
    ///
    /// [`GetRef`]: ../trait.GetRef.html
    ///
    /// # Examples
    ///```
    /// # use photonix::*;
    /// #[derive(GetRef)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(GetRef)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// #[derive(GetRef)]
    /// pub struct Address(String);
    ///
    /// //                      Level 1    Level 2  Level 3   Parent type
    /// //                          |        |        |           |
    /// impl<'a> GetRefThird<'a, Company, Address, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let company_address = john_doe.get_ref_third();
    ///
    /// assert_eq!("4 Foo Road, Bar City", company_address.as_str());
    ///
    ///```
    pub trait GetRefThird<'a, LevelOne, LevelTwo, LevelThree>
        where
            LevelTwo: GetRef<LevelThree> + 'a,
            LevelOne: GetRef<LevelTwo> + 'a,
            Self: GetRef<LevelOne> + 'a {
        fn get_ref_third(&'a self) -> &'a LevelThree {
            self.get_ref().get_ref().get_ref()
        }
    }

    /// A variant of [`Set`], reaching three levels deep in the data structure.
    ///
    /// [`Set`]: ../trait.Set.html
    ///
    /// # Examples
    ///```
    /// # use photonix::*;
    /// #[derive(Get, Modify)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(Get, Modify)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// #[derive(Get, Set)]
    /// pub struct Address(String);
    ///
    /// //            Level 1    Level 2  Level 3   Parent type
    /// //                |        |        |           |
    /// impl SetThird<Company, Address, String> for Employee {}
    /// impl GetThird<Company, Address, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let john_doe_new = john_doe.set_third(String::from("3 Baz Road, Qux City"));
    ///
    /// assert_eq!("3 Baz Road, Qux City", john_doe_new.get_third().as_str());
    ///
    ///```
    pub trait SetThird<LevelOne, LevelTwo, LevelThree>
        where
            LevelTwo: Set<LevelThree>,
            LevelOne: Modify<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn set_third(self, new_value: LevelThree) -> Self {
            self.modify(
                |level_one| level_one.modify(
                    |level_two| level_two.set(new_value)
                )
            )
        }
    }

    /// A variant of [`Modify`], reaching three levels deep in the data structure.
    ///
    /// [`Modify`]: ../trait.Modify.html
    ///
    ///```
    /// # use photonix::*;
    /// #[derive(Get, Modify)]
    /// pub struct Employee { pub name: String, pub company: Company }
    ///
    /// #[derive(Get, Modify)]
    /// pub struct Company { pub name: String, pub address: Address }
    ///
    /// #[derive(Get, Modify)]
    /// pub struct Address(String);
    ///
    /// //            Level 1    Level 2  Level 3   Parent type
    /// //                |        |        |           |
    /// impl ModifyThird<Company, Address, String> for Employee {}
    /// impl GetThird<Company, Address, String> for Employee {}
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address(String::from("4 Foo Road, Bar City")),
    ///         }
    ///     };
    ///
    /// let john_doe_new = john_doe.modify_third(|address| address.replace("4", "5"));
    ///
    /// assert_eq!("5 Foo Road, Bar City", john_doe_new.get_third().as_str());
    ///
    ///```
    pub trait ModifyThird<LevelOne, LevelTwo, LevelThree>
        where
            LevelTwo: Modify<LevelThree>,
            LevelOne: Modify<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn modify_third(self, f: impl FnOnce(LevelThree) -> LevelThree) -> Self {
            self.modify(
                |level_one| level_one.modify(
                    |level_two| level_two.modify(f)
                )
            )
        }
    }

    /// A variant of [`Get`], reaching four levels deep in the data structure.
    ///
    /// [`Get`]: ../trait.Get.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(Get)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(Get)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(Get)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(Get)]
    ///  pub struct Street { pub number: u16, pub name: String }
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: 23,
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //            Level 1   Level 2   Level 3 Level 4     Parent type
    /// //               |         |        |       |           |
    /// impl GetFourth<Company, Address, Street, String> for Employee {}
    ///
    /// assert_eq!("High street", john_doe.get_fourth().as_str());
    ///
    ///```
    pub trait GetFourth<LevelOne, LevelTwo, LevelThree, LevelFour>
        where
            LevelThree: Get<LevelFour>,
            LevelTwo: Get<LevelThree>,
            LevelOne: Get<LevelTwo>,
            Self: Get<LevelOne> + Sized {
        fn get_fourth(self) -> LevelFour {
            self.get().get().get().get()
        }
    }

    /// A variant of [`GetRef`], reaching four levels deep in the data structure.
    ///
    /// [`GetRef`]: ../trait.GetRef.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(GetRef)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(GetRef)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(GetRef)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(GetRef)]
    ///  pub struct Street { pub number: u16, pub name: String }
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: 23,
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //                      Level 1   Level 2   Level 3 Level 4     Parent type
    /// //                         |         |        |       |           |
    /// impl<'a> GetRefFourth<'a, Company, Address, Street, String> for Employee {}
    ///
    /// assert_eq!("High street", john_doe.get_ref_fourth().as_str());
    ///
    ///```
    pub trait GetRefFourth<'a, LevelOne, LevelTwo, LevelThree, LevelFour>
        where
            LevelThree: GetRef<LevelFour> + 'a,
            LevelTwo: GetRef<LevelThree> + 'a,
            LevelOne: GetRef<LevelTwo> + 'a,
            Self: GetRef<LevelOne> + 'a {
        fn get_ref_fourth(&'a self) -> &'a LevelFour {
            self.get_ref().get_ref().get_ref().get_ref()
        }
    }

    /// A variant of [`Set`], reaching four levels deep in the data structure.
    ///
    /// [`Set`]: ../trait.Set.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(Get, Modify)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(Get, Set)]
    ///  pub struct Street { pub number: u16, pub name: String }
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: 23,
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //            Level 1   Level 2   Level 3 Level 4     Parent type
    /// //               |         |        |       |           |
    /// impl SetFourth<Company, Address, Street, String> for Employee {}
    /// impl GetFourth<Company, Address, Street, String> for Employee {}
    ///
    /// let john_at_new_street = john_doe.set_fourth(String::from("Low street"));
    ///
    /// assert_eq!("Low street", john_at_new_street.get_fourth().as_str());
    ///
    ///```
    pub trait SetFourth<LevelOne, LevelTwo, LevelThree, LevelFour>
        where
            LevelThree: Set<LevelFour>,
            LevelTwo: Modify<LevelThree>,
            LevelOne: Modify<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn set_fourth(self, new_value: LevelFour) -> Self {
            self.modify(
                |level_one| level_one.modify(
                    |level_two| level_two.modify(
                        |level_three| level_three.set(new_value)
                    )
                )
            )
        }
    }

    /// A variant of [`Modify`], reaching four levels deep in the data structure.
    ///
    /// [`Modify`]: ../trait.Modify.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(Get, Modify)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Street { pub number: u16, pub name: String }
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: 23,
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //              Level 1   Level 2   Level 3 Level 4     Parent type
    /// //                 |         |        |       |           |
    /// impl ModifyFourth<Company, Address, Street, String> for Employee {}
    /// impl GetFourth<Company, Address, Street, String> for Employee {}
    ///
    /// let john_at_new_street =
    ///     john_doe.modify_fourth(|street_name| street_name.replace("street", "road"));
    ///
    /// assert_eq!("High road", john_at_new_street.get_fourth().as_str());
    ///
    ///```
    pub trait ModifyFourth<LevelOne, LevelTwo, LevelThree, LevelFour>
        where
            LevelThree: Modify<LevelFour>,
            LevelTwo: Modify<LevelThree>,
            LevelOne: Modify<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn modify_fourth(self, f: impl FnOnce(LevelFour) -> LevelFour) -> Self {
            self.modify(
                |level_one| level_one.modify(
                    |level_two| level_two.modify(
                        |level_three| level_three.modify(f)
                    )
                )
            )
        }
    }

    /// A variant of [`Get`], reaching five levels deep in the data structure.
    ///
    /// [`Get`]: ../trait.Get.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(Get)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(Get)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(Get)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(Get)]
    ///  pub struct Street { pub number: StreetNumber, pub name: String }
    ///
    ///  #[derive(Get)]
    ///  pub struct StreetNumber(u16);
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: StreetNumber(23),
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //            Level 1   Level 2   Level 3 Level     Level 5   Parent type
    /// //               |         |        |       |           |         |
    /// impl GetFifth<Company, Address, Street, StreetNumber, u16> for Employee {}
    ///
    /// assert_eq!(23, john_doe.get_fifth());
    ///
    ///```
    pub trait GetFifth<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
        where
            LevelFour: Get<LevelFive>,
            LevelThree: Get<LevelFour>,
            LevelTwo: Get<LevelThree>,
            LevelOne: Get<LevelTwo>,
            Self: Get<LevelOne> + Sized {
        fn get_fifth(self) -> LevelFive {
            self.get().get().get().get().get()
        }
    }

    /// A variant of [`GetRef`], reaching five levels deep in the data structure.
    ///
    /// [`GetRef`]: ../trait.GetRef.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(GetRef)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(GetRef)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(GetRef)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(GetRef)]
    ///  pub struct Street { pub number: StreetNumber, pub name: String }
    ///
    ///  #[derive(GetRef)]
    ///  pub struct StreetNumber(u16);
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: StreetNumber(23),
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //                      Level 1   Level 2   Level 3 Level     Level 5   Parent type
    /// //                         |         |        |       |           |         |
    /// impl<'a> GetRefFifth<'a, Company, Address, Street, StreetNumber, u16> for Employee {}
    ///
    /// assert_eq!(23, *john_doe.get_ref_fifth());
    ///
    ///```
    pub trait GetRefFifth<'a, LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
        where
            LevelFour: GetRef<LevelFive> + 'a,
            LevelThree: GetRef<LevelFour> + 'a,
            LevelTwo: GetRef<LevelThree> + 'a,
            LevelOne: GetRef<LevelTwo> + 'a,
            Self: GetRef<LevelOne> + 'a {
        fn get_ref_fifth(&'a self) -> &'a LevelFive {
            self.get_ref().get_ref().get_ref().get_ref().get_ref()
        }
    }

    /// A variant of [`Set`], reaching five levels deep in the data structure.
    ///
    /// [`Set`]: ../trait.Set.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(Get, Modify)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Street { pub number: StreetNumber, pub name: String }
    ///
    ///  #[derive(Get, Set)]
    ///  pub struct StreetNumber(u16);
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: StreetNumber(23),
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //            Level 1   Level 2   Level 3 Level     Level 5   Parent type
    /// //               |         |        |       |          |         |
    /// impl SetFifth<Company, Address, Street, StreetNumber, u16> for Employee {}
    /// impl GetFifth<Company, Address, Street, StreetNumber, u16> for Employee {}
    ///
    /// let john_doe_new_number = john_doe.set_fifth(666);
    ///
    /// assert_eq!(666, john_doe_new_number.get_fifth());
    ///
    ///```
    pub trait SetFifth<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
        where
            LevelFour: Set<LevelFive>,
            LevelThree: Modify<LevelFour>,
            LevelTwo: Modify<LevelThree>,
            LevelOne: Modify<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn set_fifth(self, new_value: LevelFive) -> Self {
            self.modify(
                |level_one| level_one.modify(
                    |level_two| level_two.modify(
                        |level_three| level_three.modify(
                            |level_four| level_four.set(new_value)
                        )
                    )
                )
            )
        }
    }

    /// A variant of [`Modify`], reaching five levels deep in the data structure.
    ///
    /// [`Modify`]: ../trait.Modify.html
    ///
    /// # Examples
    ///```
    ///  # use photonix::*;
    ///  #[derive(Get, Modify)]
    ///  pub struct Employee { pub name: String, pub company: Company }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Company { pub name: String, pub address: Address }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Address { pub city: String, pub street: Street }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct Street { pub number: StreetNumber, pub name: String }
    ///
    ///  #[derive(Get, Modify)]
    ///  pub struct StreetNumber(u16);
    ///
    /// let john_doe = Employee {
    ///     name: String::from("John Doe"),
    ///     company: Company {
    ///         name: String::from("Acme Corporation"),
    ///         address: Address {
    ///                 city: String::from("London"),
    ///                 street: Street {
    ///                     number: StreetNumber(23),
    ///                     name: String::from("High street"),
    ///                 }
    ///             },
    ///         }
    ///     };
    ///
    /// //              Level 1   Level 2   Level 3 Level     Level 5   Parent type
    /// //                 |         |        |       |          |         |
    /// impl ModifyFifth<Company, Address, Street, StreetNumber, u16> for Employee {}
    /// impl GetFifth<Company, Address, Street, StreetNumber, u16> for Employee {}
    ///
    /// let john_doe_new_number = john_doe.modify_fifth(|num: u16| num * 10);
    ///
    /// assert_eq!(230, john_doe_new_number.get_fifth());
    ///
    ///```
    pub trait ModifyFifth<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
        where
            LevelFour: Modify<LevelFive>,
            LevelThree: Modify<LevelFour>,
            LevelTwo: Modify<LevelThree>,
            LevelOne: Modify<LevelTwo>,
            Self: Modify<LevelOne> + Sized {
        fn modify_fifth(self, f: impl FnOnce(LevelFive) -> LevelFive) -> Self {
            self.modify(
                |level_one| level_one.modify(
                    |level_two| level_two.modify(
                        |level_three| level_three.modify(
                            |level_four| level_four.modify(f)
                        )
                    )
                )
            )
        }
    }
}

/// Auto-implements different [`composites`] of getters, setters, and modifiers.
///
/// The requirement is that the elements should have an implementation of [`Get`], [`GetRef`], [`Modify`], and [`Set`] with the target type at the next level (see definitions of [`composites`] for details).
///
/// Since [`Get`] and [`GetRef`] are traits for structs, this macro is recommended to use with structs.
///
/// [`Get`]: focus/trait.Get.html
/// [`GetRef`]: focus/trait.GetRef.html
/// [`Modify`]: focus/trait.Modify.html
/// [`Set`]: focus/trait.Set.html
/// [`composites`]: focus/composites/index.html
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(Get, GetRef, Set, Modify)]
/// pub struct Employee { pub name: String, pub company: Company }
/// #[derive(Get, GetRef, Set, Modify)]
/// pub struct Company { pub name: String, pub address: Address }
/// #[derive(Get, GetRef, Set, Modify)]
/// pub struct Address { pub city: String }
///
/// let john_doe = Employee {
///     name: String::from("John Doe"),
///     company: Company {
///         name: String::from("Awesome Inc"),
///         address: Address {
///             city: String::from("London"),
///         }
///     }
/// };
///
/// //      Parent type     Level 1    Level 2
/// //          |              |         |
/// zoom_all![Employee => Company => String];
///
/// //      Parent type     Level 1    Level 2    Level 3
/// //          |              |         |          |
/// zoom_all![Employee => Company => Address => String];
///
/// assert_eq!("Awesome Inc", john_doe.get_ref_second().as_str());
/// assert_eq!("London", john_doe.get_ref_third().as_str());
///
/// let john_doe_company_changed_name = john_doe.modify_second(|s| s.replace("Inc", "Corp"));
///
/// assert_eq!("Awesome Corp", john_doe_company_changed_name.get_ref_second().as_str());
///
/// let john_doe_relocated = john_doe_company_changed_name.set_third(String::from("Paris"));
///
/// assert_eq!("Paris", john_doe_relocated.get_ref_third().as_str());
///
///```
#[macro_export]
macro_rules! zoom_all {
    ($outer:ty => $first:ty => $second:ty) => {
        impl<'a> GetRefSecond<'a, $first, $second> for $outer {}
        impl GetSecond<$first, $second> for $outer {}
        zoom!($outer => $first => $second);
    };

    ($outer:ty => $first:ty => $second:ty => $third:ty) => {
        impl<'a> GetRefThird<'a, $first, $second, $third> for $outer {}
        impl GetThird<$first, $second, $third> for $outer {}
        zoom!($outer => $first => $second => $third);
    };

    ($outer:ty => $first:ty => $second:ty => $third:ty => $fourth:ty) => {
        impl<'a> GetRefFourth<'a, $first, $second, $third, $fourth> for $outer {}
        impl GetFourth<$first, $second, $third, $fourth> for $outer {}
        zoom!($outer => $first => $second => $third => $fourth);
    };

    ($outer:ty => $first:ty => $second:ty => $third:ty => $fourth:ty => $fifth:ty) => {
        impl<'a> GetRefFifth<'a, $first, $second, $third, $fourth, $fifth> for $outer {}
        impl GetFifth<$first, $second, $third, $fourth, $fifth> for $outer {}
        zoom!($outer => $first => $second => $third => $fourth => $fifth);
    };
}

/// Auto-implements [`composites`] of setters and modifiers.
///
/// The requirement is that the elements should have an implementation of [`Modify`] and [`Set`] with the target type at the next level (see definitions of [`composites`] for details).
///
/// You can use this macro with both structs and enums.
///
/// [`Modify`]: focus/trait.Modify.html
/// [`Set`]: focus/trait.Set.html
/// [`composites`]: focus/composites/index.html
///
/// # Examples
///```
/// # use photonix::*;
/// #[derive(GetOption, Set, Modify)]
/// pub enum User {
///     Admin,
///     RegularUser(Employee),
/// }
///
/// #[derive(Get, Set, Modify)]
/// pub struct Employee { pub name: String, pub company: Company }
///
/// #[derive(Get, Set, Modify)]
/// pub struct Company { pub name: String, pub address: Address }
///
/// #[derive(Get, Set, Modify)]
/// pub struct Address(String);
///
/// let john_doe = || Employee {
///     name: String::from("John Doe"),
///     company: Company {
///         name: String::from("Acme Corporation"),
///         address: Address(String::from("4 Foo Road, Bar City")),
///         }
///     };
///
/// let john_as_user = || User::RegularUser(john_doe());
///
/// //  Parent type   Level 1   Level 2    Level 3    Level 4
/// //      |           |          |         |         |
/// zoom![User => Employee => Company => Address => String];
///
/// let john_at_new_address = john_as_user().modify_fourth(|s| s.replace("4", "5"));
///
/// let maybe_john = john_at_new_address.get_option();
///
/// assert_eq!(
///     Some(String::from("5 Foo Road, Bar City")),
///     maybe_john.map(|john| john.company.address.0)
/// );
///
/// //  Parent type   Level 1   Level 2
/// //      |           |        |
/// zoom![User => Employee => String];
///
/// let frank_doe_as_user = john_as_user().set_second(String::from("Frank Doe"));
///
/// assert_eq!(
///     Some(String::from("Frank Doe")),
///     frank_doe_as_user.get_option().map(|employee| employee.name)
/// );
///
///```
///
#[macro_export]
macro_rules! zoom {
    ($outer:ty => $first:ty => $second:ty) => {
        impl SetSecond<$first, $second> for $outer {}
        impl ModifySecond<$first, $second> for $outer {}
    };

    ($outer:ty => $first:ty => $second:ty => $third:ty) => {
        impl SetThird<$first, $second, $third> for $outer {}
        impl ModifyThird<$first, $second, $third> for $outer {}
    };

    ($outer:ty => $first:ty => $second:ty => $third:ty => $fourth:ty) => {
        impl SetFourth<$first, $second, $third, $fourth> for $outer {}
        impl ModifyFourth<$first, $second, $third, $fourth> for $outer {}
    };

    ($outer:ty => $first:ty => $second:ty => $third:ty => $fourth:ty => $fifth:ty) => {
        impl SetFifth<$first, $second, $third, $fourth, $fifth> for $outer {}
        impl ModifyFifth<$first, $second, $third, $fourth, $fifth> for $outer {}
    };
}
