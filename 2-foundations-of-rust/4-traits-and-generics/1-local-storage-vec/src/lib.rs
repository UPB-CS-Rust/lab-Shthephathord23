pub enum LocalStorageVec<T, const N: usize> {
    Stack {
        buf: [T; N],
        len: usize,
    },
    Heap(Vec<T>),
}

impl<T: Default, const N: usize> LocalStorageVec<T, N> {
    pub fn new() -> Self {
        Self::Stack {
            buf: [(); N].map(|_| T::default()),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Stack { len, .. } => *len,
            Self::Heap(v) => v.len(),
        }
    }

    pub fn push(&mut self, value: T)
    where
        T: Clone,
    {
        match self {
            Self::Stack { buf, len } if *len < N => {
                buf[*len] = value;
                *len += 1;
            }
            _ => {
                let mut heap_vec = match std::mem::replace(self, Self::Heap(Vec::new())) {
                    Self::Stack { buf, len } => {
                        let mut v = Vec::with_capacity(len + 1);
                        v.extend_from_slice(&buf[..len]);
                        v
                    }
                    Self::Heap(v) => v,
                };
                heap_vec.push(value);
                *self = Self::Heap(heap_vec);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Stack { buf, len } if *len > 0 => {
                *len -= 1;
                Some(std::mem::replace(&mut buf[*len], T::default()))
            }
            Self::Heap(v) => v.pop(),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<T, const N: usize, const M: usize> From<[T; N]> for LocalStorageVec<T, M>
where
    T: Default + Clone,
{
    fn from(array: [T; N]) -> Self {
        if N <= M {
            let mut it = array.into_iter();
            Self::Stack {
                buf: [(); M].map(|_| it.next().unwrap_or_default()),
                len: N,
            }
        } else {
            Self::Heap(Vec::from(array))
        }
    }
}

impl<T: Default + Clone, const N: usize> From<Vec<T>> for LocalStorageVec<T, N> {
    fn from(vec: Vec<T>) -> Self {
        if vec.len() <= N {
            let mut buf = [(); N].map(|_| T::default());
            let len = vec.len();
            buf[..len].clone_from_slice(&vec);  // Use clone_from_slice instead of copy_from_slice
            Self::Stack { buf, len }
        } else {
            Self::Heap(vec)
        }
    }
}


#[cfg(test)]
mod test {
    use crate::LocalStorageVec;

    #[test]
    #[ignore]
    fn it_compiles() {
        let vec: LocalStorageVec<u32, 10> = loop {};
        match vec {
            LocalStorageVec::Stack { buf, len } => {
                let _buf: [u32; 10] = buf;
                let _len: usize = len;
            }
            LocalStorageVec::Heap(v) => {
                let _v: Vec<u32> = v;
            }
        }
    }

    #[test]
    fn it_from_vecs() {
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::from(vec![1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));

        let vec: LocalStorageVec<usize, 2> = LocalStorageVec::from(vec![1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
    }

    #[test]
    fn it_constructs() {
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::new();
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 0 }));
    }

    #[test]
    fn it_pushes() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::new();
        for value in 0..128 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Stack { len: 128, .. }));
        for value in 128..256 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Heap(v) if v.len() == 256));
    }

    #[test]
    fn it_pops() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        for _ in 0..128 {
            assert_eq!(vec.pop(), Some(0));
        }
        assert_eq!(vec.pop(), None);
    }
}
