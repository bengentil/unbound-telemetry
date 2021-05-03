use crate::super_enum;

super_enum! {
    enum Class {
        In => (0, "CLASS0"),
        In => (1, "IN"),
        Ch => (3, "CH"),
        Hs => (4, "HS"),
        Class5 => (5, "CLASS5"),
        None => (0xFE, "NONE"),
        Any => (0xFF, "ANY"),
    }
}
