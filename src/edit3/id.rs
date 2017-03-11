
#[derive(Eq,PartialEq,Copy,Clone)]
pub struct ID{
    pub storage_key:usize,
    pub unique_id:usize,
}

impl ID{
    pub fn new(storage_key:usize, unique_id:usize) -> Self {
        ID{
            storage_key:storage_key,
            unique_id:unique_id,
        }
    }

    pub fn zeroed() -> Self {
        ID{
            storage_key:0,
            unique_id:0,
        }
    }
}
