extern "C" {
    fn malloc(s: usize) -> *mut u8;
    fn calloc(s: usize, n:usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn realloc(ptr: *mut u8, s: usize)-> *mut u8;
}
use core::fmt::Debug;

pub struct Vec<T> {
    ptr: *mut T,
    pub len: usize,
    elem_size: usize
}
impl<T: core::fmt::Debug> Vec<T> {
    pub fn new() -> Self {
        let size = core::mem::size_of::<T>();

        let ptr = unsafe 
        {
            malloc(size) as *mut T
        };
        Self { len: 0, ptr: ptr, elem_size:size}
    }
    pub fn push(&mut self, value: T)
    {
        if self.len == 0
        {
            self.memcpy(value, self.ptr);
            self.len += 1;
        }
        else 
        {
            let ptr = unsafe 
            {
                realloc(self.ptr as *mut u8, (self.len+1)*self.elem_size) as *mut T
            };
            if ptr != self.ptr
            {
                println!("NEED REALLOC");
                println!("PTR = {:?}", self.ptr);
                println!("NEW PTR = {:?}", ptr);
            }
            self.ptr = ptr;
            self.memcpy(value, unsafe {self.ptr.add(self.len)});
            self.len += 1;
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            unsafe { Some(&*(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn get_mut(&self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            unsafe { Some(&mut *(self.ptr.add(idx))) }
        } else {
            None
        }
    }

    fn memcpy(&self, value: T, ptr: *mut T) {      
        unsafe { *ptr = value; }
    }
}

impl<T: core::fmt::Debug> Debug for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) 
    -> 
    core::fmt::Result 
    {
        let mut res = f.debug_list();
        for i in 0..self.len
        {
            res.entries(self.get(i));
        }
        res.finish()
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe 
        {
            free(self.ptr as *mut u8)
        };
    }
}
impl<T: core::fmt::Debug> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

/*
impl<T> std::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}*/