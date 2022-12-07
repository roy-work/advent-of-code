pub trait ToTuple<T, I: Iterator<Item = T>> {
    fn to_tuple(i: &mut I) -> Result<Self, ToTupleErr> where Self: Sized;
}

impl<T, I: Iterator<Item = T>> ToTuple<T, I> for (T, T) {
    fn to_tuple(i: &mut I) -> Result<Self, ToTupleErr> {
        let mut need = NeedNItems::new(i, 2);
        let a = need.next()?;
        let b = need.next()?;
        need.finish()?;
        Ok((a, b))
    }
}

impl<T, I: Iterator<Item = T>> ToTuple<T, I> for (T, T, T) {
    fn to_tuple(i: &mut I) -> Result<Self, ToTupleErr> {
        let mut need = NeedNItems::new(i, 3);
        let a = need.next()?;
        let b = need.next()?;
        let c = need.next()?;
        need.finish()?;
        Ok((a, b, c))
    }
}

impl<T, I: Iterator<Item = T>> ToTuple<T, I> for (T, T, T, T) {
    fn to_tuple(i: &mut I) -> Result<Self, ToTupleErr> {
        let mut need = NeedNItems::new(i, 4);
        let a = need.next()?;
        let b = need.next()?;
        let c = need.next()?;
        let d = need.next()?;
        need.finish()?;
        Ok((a, b, c, d))
    }
}

pub trait IterExtToTuple<R> {
    fn to_tuple(&mut self) -> Result<R, ToTupleErr>;
}

impl<T, I: Iterator<Item = T>> IterExtToTuple<(T, T)> for I {
    fn to_tuple(&mut self) -> Result<(T, T), ToTupleErr> {
        ToTuple::to_tuple(self)
    }
}

impl<T, I: Iterator<Item = T>> IterExtToTuple<(T, T, T)> for I {
    fn to_tuple(&mut self) -> Result<(T, T, T), ToTupleErr> {
        ToTuple::to_tuple(self)
    }
}

impl<T, I: Iterator<Item = T>> IterExtToTuple<(T, T, T, T)> for I {
    fn to_tuple(&mut self) -> Result<(T, T, T, T), ToTupleErr> {
        ToTuple::to_tuple(self)
    }
}

struct NeedNItems<I> {
    need: usize,
    got: usize,
    iter: I,
}

impl<I: Iterator> NeedNItems<I> {
    fn new(iter: I, need: usize) -> NeedNItems<I> {
        NeedNItems {
            need,
            got: 0,
            iter,
        }
    }

    fn next(&mut self) -> Result<I::Item, ToTupleErr> {
        match self.iter.next() {
            Some(item) => {
                self.got += 1;
                Ok(item)
            }
            None => Err(ToTupleErr::TooFew {
                need: self.need,
                got: self.got,
            }),
        }
    }

    fn finish(mut self) -> Result<(), ToTupleErr> {
        match self.iter.next() {
            Some(_) => Err(ToTupleErr::TooMany(self.need)),
            None => Ok(())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ToTupleErr {
    #[error("needed {need} items, but only got {got}")]
    TooFew {
        need: usize,
        got: usize,
    },
    #[error("extra items; only needed {0} items")]
    TooMany(usize),
}

#[cfg(test)]
mod tests {
    use super::{IterExtToTuple, ToTupleErr};

    #[test]
    fn test_to_tuple() {
        let input_str = "abc def";
        let t: (&str, &str) = input_str.split(' ').to_tuple().unwrap();
        assert!(t.0 == "abc");
        assert!(t.1 == "def");

        let input_str = "abc def ghi";
        let result: Result<(_, _), _> = input_str.split(' ').to_tuple();
        match result {
            Ok(_) => panic!("should have failed, but got: {:?}", result),
            Err(ToTupleErr::TooMany(2)) => (),
            Err(_) => panic!("wrong error."),
        }
        let result: Result<(_, _, _, _), _> = input_str.split(' ').to_tuple();
        match result {
            Ok(_) => panic!("should have failed, but got: {:?}", result),
            Err(ToTupleErr::TooFew { need: 4, got: 3}) => (),
            Err(err) => panic!("wrong error: {:?}", err),
        }
    }
}
