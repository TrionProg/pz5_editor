use std;

use std::sync::Arc;
use std::sync::RwLock;
use ID;


pub trait SlabElement{
    fn set_id(&mut self,id:ID);
    fn get_id(&self) -> ID;
}

struct GrowableSlabInner<T:SlabElement>{
    slots:Vec< Option<Arc<T>> >,
    last:usize,
    free:usize,
    unique_id:usize,
}

pub struct GrowableSlab<T:SlabElement>{
     inner:RwLock<GrowableSlabInner<T>>,
}

impl<T:SlabElement> GrowableSlab<T> {
    pub fn with_capacity(mut size:usize) -> Self {
        if size==0 {
            size=1;
        }

        let mut slots=Vec::with_capacity(size);
        slots.resize(size,None);

        let inner=GrowableSlabInner{
            slots:slots,
            last:0,
            free:0,
            unique_id:1,
        };

        GrowableSlab{
            inner:RwLock::new(inner),
        }
    }

    pub fn insert(&self, mut element:T) -> Arc<T> {
        let mut inner=self.inner.write().unwrap();

        let len=inner.slots.len();
        if inner.free==len {
            inner.slots.resize(len*2,None);
        }

        let id=ID::new(inner.free, inner.unique_id);
        element.set_id(id);

        let element_arc=Arc::new(element);
        let free=inner.free;
        inner.slots[free]=Some(element_arc.clone());

        inner.unique_id+=1;
        inner.free+=1;

        while inner.free<inner.slots.len() && inner.slots[inner.free].is_some() {
            inner.free+=1;
        }

        if inner.free>inner.last {
            inner.last+=1;
        }

        element_arc
    }

    pub fn remove(&self, id:ID) -> Option< Arc<T> > {
        let mut inner=self.inner.write().unwrap();

        if id.storage_key>inner.last {
            return None;
        }

        let ret_val=match inner.slots[id.storage_key] {
            Some( ref element ) => {
                if element.get_id()!=id {
                    return None;
                }

                element.clone()
            },
            None => return None,
        };

        inner.slots[id.storage_key]=None;

        inner.free=id.storage_key;

        if inner.free==inner.last {
            inner.last-=1;

            if inner.last*2 < inner.slots.len() {
                let new_len=inner.last+1;
                inner.slots.truncate(new_len);
            }
        }

        Some(ret_val)
    }

    pub fn get(&self, id:ID) -> Option<Arc<T>> {
        let inner=self.inner.write().unwrap();

        if id.storage_key>inner.last {
            return None;
        }

        match inner.slots[id.storage_key] {
            Some( ref element ) => {
                if element.get_id()!=id {
                    None
                }else{
                    Some( element.clone() )
                }
            },
            None => None
        }
    }
}
