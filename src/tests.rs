#[cfg(test)]
mod tests {
    use crate::lpftokens::{VStackItem, HStackItem};

    #[test]
    fn one_hstackitem(){
        assert_eq!(
            VStackItem {
                inner: vec![
                    HStackItem::new(0.4)
                ]
            }, VStackItem::new("[0.2]")
        );
    }
}