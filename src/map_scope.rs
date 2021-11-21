use std::cell::RefCell;

use druid::{
    widget::{Scope, ScopeTransfer},
    Data, Widget,
};

struct MyTransfer<T, U, F, P> {
    format: F,
    parse: P,
    cache: RefCell<Option<(T, U)>>,
}

impl<T, U, F, P> ScopeTransfer for MyTransfer<T, U, F, P>
where
    T: Data,
    U: Data,
    F: Fn(&T) -> U,
    P: Fn(&U) -> T,
{
    type In = T;
    type State = U;

    fn read_input(&self, state: &mut U, inner: &T) {
        let mut cached = self.cache.borrow_mut();
        if cached
            .as_mut()
            .map_or(true, |(cached, _)| !cached.same(inner))
        {
            *state = (self.format)(inner);
            *cached = Some((inner.clone(), state.clone()));
        }
    }

    fn write_back_input(&self, state: &U, inner: &mut T) {
        let mut cached = self.cache.borrow_mut();
        if cached
            .as_mut()
            .map_or(true, |(_, cached)| !cached.same(state))
        {
            *inner = (self.parse)(state);
            *cached = Some((inner.clone(), state.clone()));
        }
    }
}

pub fn map<T, U>(
    inner: impl Widget<U>,
    format: impl Fn(&T) -> U + Clone,
    parse: impl Fn(&U) -> T,
) -> impl Widget<T>
where
    T: Data,
    U: Data,
{
    Scope::from_function(
        {
            let format = format.clone();
            move |value| format(&value)
        },
        MyTransfer {
            format,
            parse,
            cache: RefCell::new(None),
        },
        inner,
    )
}

// ============================

// struct IndexTransfer<T, S, W> {
//     select_index: S,
//     write_back: W,
//     cache: RefCell<Option<(T, usize)>>,
// }

// impl<T, S, W> ScopeTransfer for IndexTransfer<T, S, W>
// where
//     T: Data,
//     S: Fn(&T) -> usize,
//     W: Fn(&mut T, usize),
// {
//     type In = T;
//     type State = usize;

//     fn read_input(&self, state: &mut usize, inner: &T) {
//         let mut cached = self.cache.borrow_mut();
//         if cached
//             .as_mut()
//             .map_or(true, |(cached, _)| !cached.same(inner))
//         {
//             *state = (self.select_index)(inner);
//             *cached = Some((inner.clone(), *state));
//         }
//     }

//     fn write_back_input(&self, state: &usize, inner: &mut T) {
//         let mut cached = self.cache.borrow_mut();
//         if cached
//             .as_mut()
//             .map_or(true, |(_, cached)| !cached.same(state))
//         {
//             (self.write_back)(inner, *state);
//             *cached = Some((inner.clone(), *state));
//         }
//     }
// }

// pub fn index_map<T>(
//     inner: impl Widget<usize>,
//     select_index: impl Fn(&T) -> usize + Clone,
//     write_back: impl Fn(&mut T, usize),
// ) -> impl Widget<T>
// where
//     T: Data,
// {
//     Scope::from_function(
//         {
//             let select_index = select_index.clone();
//             move |value| select_index(&value)
//         },
//         IndexTransfer {
//             select_index,
//             write_back,
//             cache: RefCell::new(None),
//         },
//         inner,
//     )
// }
