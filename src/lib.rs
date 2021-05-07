
// #![no_std]

pub struct RingPtr {
    top: usize,
    end: usize,
    max: usize,
}
impl RingPtr {
    fn new(len: usize) -> RingPtr {
        RingPtr {
            top: 0,
            end: 0,
            max: len,
        }
    }
    fn push(&mut self) -> Option<usize> {
        let mut next = self.top + 1;
        if next >= self.max { next = 0; }
        if next == self.end { return None; }
        let rslt = self.top;
        self.top += 1;
        if self.top >= self.max { self.top = 0; }
        Some(rslt)
    }
    fn pop(&mut self) -> Option<usize> {
        if self.top == self.end { return None; }
        let rslt = self.end;
        self.end += 1;
        if self.end >= self.max { self.end = 0; }
        Some(rslt)
    }
    fn get_left(&self) -> usize {
        if self.end > self.top {
            return self.end - self.top - 1;
        }
        self.max - self.top + self.end - 1
    }
}

const MAX: usize = 10;

struct RingBuff {
    data: [u32; MAX],
    ptr: RingPtr,
}
impl RingBuff {
    fn new() -> RingBuff {
        RingBuff {
            data: [0; MAX],
            ptr: RingPtr::new(MAX),
        }
    }
    fn push(&mut self, v: u32) -> Option<u32> {
        if let Some(n) = self.ptr.push() {
            self.data[n] = v;
            return Some(v);
        }
        None
    }
    fn pop(&mut self) -> Option<u32> {
        if let Some(n) = self.ptr.pop() {
            return Some(self.data[n]);
        }
        None
    }
    fn get_left(&self) -> usize {
        self.ptr.get_left()
    }
}

fn debug_push_print(v: Option<u32>) {
    match v {
        Some(n) => println!("PUSH {}", n),
        None => println!("PUSH Full")
    }
}
fn debug_pop_print(v: Option<u32>) {
    match v {
        Some(n) => println!("POP {}", n),
        None => println!("POP Empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ringbuff_empty() {
        let mut rb: RingBuff = RingBuff::new();
        assert_eq!(None, rb.pop());
    }
    #[test]
    fn test_ringbuff_normal() {
        let mut rb: RingBuff = RingBuff::new();
        assert_eq!(10, rb.push(10).unwrap());
        let rslt = rb.pop();
        assert_eq!(10, rslt.unwrap());
        assert_eq!(20, rb.push(20).unwrap());
        let rslt = rb.pop();
        assert_eq!(20, rslt.unwrap());
        assert_eq!(30, rb.push(30).unwrap());
        assert_eq!(40, rb.push(40).unwrap());
        let rslt = rb.pop();
        assert_eq!(30, rslt.unwrap());
        let rslt = rb.pop();
        assert_eq!(40, rslt.unwrap());
        let rslt = rb.pop();
        assert_eq!(None, rslt);
    }
    #[test]
    fn test_ringbuff_full() {
        let mut rb: RingBuff = RingBuff::new();
        for i in 2..MAX {
            rb.push(10);
        }
        assert_eq!(123, rb.push(123).unwrap());
        assert_eq!(None, rb.push(321));
        assert_eq!(None, rb.push(345));
    }
    #[test]
    fn test_ringbuff_get_left() {
        let mut rb: RingBuff = RingBuff::new();
        assert_eq!(9, rb.get_left());
        rb.push(10);
        assert_eq!(8, rb.get_left());
        rb.push(9);
        for i in 0..6 {
            rb.push(i);
        }
        assert_eq!(1, rb.get_left());
        rb.push(10);
        assert_eq!(0, rb.get_left());
        rb.pop();
        assert_eq!(1, rb.get_left());
        for i in 0..3 {
            rb.pop();
        }
        assert_eq!(4, rb.get_left());
        for i in 0..3 {
            rb.push(10);
        }
        assert_eq!(1, rb.get_left());
    }
}
