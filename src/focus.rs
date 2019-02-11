pub trait Get<Value> {
    fn get(self) -> Value;
}

pub trait GetRef<Value> {
    fn get_ref(&self) -> &Value;
}

pub trait Set<Value> {
    fn set(self, new_value: Value) -> Self;
}

pub trait Modify<Value> {
    fn modify(self, f: impl FnOnce(Value) -> Value) -> Self;
}

pub trait GetOption<Value> {
    fn get_option(self) -> Option<Value>;
}

pub trait SetOption<Value>
    where Self: Sized {
    fn set_option(self, new_value: Value) -> Option<Self>;
}

pub trait ModifyOption<Value>
    where Self: Sized {
    fn modify_option(self, f: impl FnOnce(Value) -> Value) -> Option<Self>;
}

pub trait ReverseGet<Value> {
    fn reverse_get(value: Value) -> Self;
}

pub trait ExtractFocus<Value> {
    fn extract(self) -> Value;
}

pub trait GetSecond<LevelOne, LevelTwo>
    where
        LevelOne: Get<LevelTwo>,
        Self: Get<LevelOne> + Sized {
    fn get_second(self) -> LevelTwo {
        self.get().get()
    }
}

pub trait GetRefSecond<'a, LevelOne, LevelTwo>
    where
        LevelOne: GetRef<LevelTwo> + 'a,
        Self: GetRef<LevelOne> + 'a {
    fn get_ref_second(&'a self) -> &'a LevelTwo {
        self.get_ref().get_ref()
    }
}

pub trait SetSecond<LevelOne, LevelTwo>
    where
        LevelOne: Set<LevelTwo>,
        Self: Modify<LevelOne> + Sized {
    fn set_second(self, new_value: LevelTwo) -> Self {
        self.modify(|level_one| level_one.set(new_value))
    }
}

pub trait ModifySecond<LevelOne, LevelTwo>
    where
        LevelOne: Modify<LevelTwo>,
        Self: Modify<LevelOne> + Sized {
    fn modify_second(self, f: impl FnOnce(LevelTwo) -> LevelTwo) -> Self {
        self.modify(|level_one| level_one.modify(f))
    }
}

pub trait GetThird<LevelOne, LevelTwo, LevelThree>
    where
        LevelTwo: Get<LevelThree>,
        LevelOne: Get<LevelTwo>,
        Self: Get<LevelOne> + Sized {
    fn get_third(self) -> LevelThree {
        self.get().get().get()
    }
}

pub trait GetRefThird<'a, LevelOne, LevelTwo, LevelThree>
    where
        LevelTwo: GetRef<LevelThree> + 'a,
        LevelOne: GetRef<LevelTwo> + 'a,
        Self: GetRef<LevelOne> + 'a {
    fn get_ref_third(&'a self) -> &'a LevelThree {
        self.get_ref().get_ref().get_ref()
    }
}

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

pub trait GetFifth<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
    where
        LevelFour: Get<LevelFive>,
        LevelThree: Get<LevelFour>,
        LevelTwo: Get<LevelThree>,
        LevelOne: Get<LevelTwo>,
        Self: Get<LevelOne> + Sized {
    fn get_ref_fifth(self) -> LevelFive {
        self.get().get().get().get().get()
    }
}

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
