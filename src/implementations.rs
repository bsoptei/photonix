use crate::*;

impl<T> Set<T> for Option<T> {
    fn set(self, new_value: T) -> Self {
        self.map(|_| new_value)
    }
}

impl<T> Modify<T> for Option<T> {
    fn modify(self, f: impl FnOnce(T) -> T) -> Self {
        self.map(f)
    }
}

impl<LevelOne, LevelTwo>
SetSecond<LevelOne, LevelTwo>
for Option<LevelOne>
    where LevelOne: Set<LevelTwo> {}

impl<LevelOne, LevelTwo, LevelThree>
SetThird<LevelOne, LevelTwo, LevelThree>
for Option<LevelOne>
    where LevelOne: Modify<LevelTwo>,
          LevelTwo: Set<LevelThree> {}

impl<LevelOne, LevelTwo, LevelThree, LevelFour>
SetFourth<LevelOne, LevelTwo, LevelThree, LevelFour>
for Option<LevelOne>
    where LevelOne: Modify<LevelTwo>,
          LevelTwo: Modify<LevelThree>,
          LevelThree: Set<LevelFour> {}

impl<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
SetFifth<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
for Option<LevelOne>
    where LevelOne: Modify<LevelTwo>,
          LevelTwo: Modify<LevelThree>,
          LevelThree: Modify<LevelFour>,
          LevelFour: Set<LevelFive> {}


impl<LevelOne, LevelTwo>
ModifySecond<LevelOne, LevelTwo>
for Option<LevelOne>
    where LevelOne: Modify<LevelTwo> {}

impl<LevelOne, LevelTwo, LevelThree>
ModifyThird<LevelOne, LevelTwo, LevelThree>
for Option<LevelOne>
    where LevelOne: Modify<LevelTwo>,
          LevelTwo: Modify<LevelThree> {}

impl<LevelOne, LevelTwo, LevelThree, LevelFour>
ModifyFourth<LevelOne, LevelTwo, LevelThree, LevelFour>
for Option<LevelOne>
    where LevelOne: Modify<LevelTwo>,
          LevelTwo: Modify<LevelThree>,
          LevelThree: Modify<LevelFour> {}

impl<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
ModifyFifth<LevelOne, LevelTwo, LevelThree, LevelFour, LevelFive>
for Option<LevelOne>
    where LevelOne: Modify<LevelTwo>,
          LevelTwo: Modify<LevelThree>,
          LevelThree: Modify<LevelFour>,
          LevelFour: Modify<LevelFive> {}

