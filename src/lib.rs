#![allow(unused_variables, dead_code)]
pub mod first;

#[cfg(test)]
mod test {
    use crate::first::List;
    #[test]
    fn test_pop_push() {
        let mut ourlist = List::new();

        ourlist.push(1);
        ourlist.push(2);
        ourlist.push(3);

        ourlist.pop();
        ourlist.pop();

        assert_eq!(ourlist.pop(), Some(1));
        assert_eq!(ourlist.pop(), None);

        ourlist.push(4);

        assert_eq!(ourlist.pop(), Some(4));
    }

    #[test]
    fn test_vecify() {
        let mut ourlist = List::new();
        ourlist.push(1);
        ourlist.push(2);
        ourlist.push(3);
        ourlist.push(4);

        assert_eq!(ourlist.vecify_list(), [4, 3, 2, 1]);
    }
}
