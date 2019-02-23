use std::marker::PhantomData;
use std::ptr;
use std::sync::Arc;

struct Node<T> {
    data: T,
    next: Option<Arc<Node<T>>>,
}

pub struct ArcList<T> {
    inner: Option<Arc<Node<T>>>,
}

impl<T> ArcList<T> {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn insert(self, data: T) -> Self {
        Self {
            inner: Some(Arc::new(Node {
                data,
                next: self.inner,
            })),
        }
    }

    pub fn iter<'s>(&'s self) -> ArcListIter<'s, T> {
        ArcListIter {
            inner: self
                .inner
                .as_ref()
                .map(|inner| &**inner as *const _)
                .unwrap_or(ptr::null()),
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for ArcList<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct ArcListIter<'s, T> {
    inner: *const Node<T>,
    _marker: PhantomData<&'s T>,
}

impl<'s, T> Iterator for ArcListIter<'s, T> {
    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = unsafe { self.inner.as_ref()? };
        self.inner = inner
            .next
            .as_ref()
            .map(|inner| &**inner as *const _)
            .unwrap_or(ptr::null());
        Some(&inner.data)
    }
}
