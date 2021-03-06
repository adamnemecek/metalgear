use crate::{BufferAllocator, MemAlign};

pub struct GPUVar<T: Copy> {
    device: metal::Device,
    inner: metal::Buffer,
    // mem_align: MemAlign<T>,
    phantom: std::marker::PhantomData<T>,
}

// impl<T: Copy> GPUVar<T> {
//     #[inline]
//     pub(crate) fn inner(&self) -> &metal::BufferRef {
//         &self.inner
//     }
// }

impl<T: Copy> GPUVar<T> {
    pub fn label(&self) -> &str {
        self.inner.label()
    }

    pub fn set_label(&self, label: &str) {
        self.inner.set_label(label)
    }

    pub fn add_debug_marker(&self, label: &str, range: std::ops::Range<u64>) {
        let range = metal::NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        self.inner.add_debug_marker(label, range)
    }

    pub fn remove_all_debug_markers(&self) {
        self.inner.remove_all_debug_markers()
    }
}

impl<T: Copy> GPUVar<T> {
    pub fn with_value(device: &metal::DeviceRef, value: T) -> Self {
        let mem_align = MemAlign::<T>::new(1);
        let inner = device.new_mem(
            mem_align,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        let mut ret = Self {
            device: device.to_owned(),
            inner,
            phantom: Default::default(),
        };
        *ret = value;
        ret
    }

    pub fn with_value1(value: T) -> Self {
        let mut ret = Self::new();
        *ret = value;
        ret
    }

    pub fn new() -> Self {
        let device = metal::Device::system_default().unwrap();
        let mem_align = MemAlign::<T>::new(1);
        let inner = device.new_mem(
            mem_align,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        let ret = Self {
            device: device.to_owned(),
            inner,
            phantom: Default::default(),
        };
        ret
    }

    #[inline]
    pub fn element_size() -> usize {
        std::mem::size_of::<T>()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.inner.contents() as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.inner.contents() as *mut T
    }

    pub fn to_owned(&self) -> Self {
        Self {
            device: self.device.to_owned(),
            inner: self.inner.to_owned(),
            phantom: self.phantom,
        }
    }

    // #[inline]
    // pub fn value(&self) -> T {
    //     unsafe {
    //         *self.as_ptr()
    //     }
    // }

    // #[inline]
    // pub fn set_value(&mut self, value: T) {
    //     unsafe {
    //         std::ptr::copy(
    //             &value,
    //             self.as_mut_ptr(),
    //             Self::element_size()
    //         );
    //     }
    // }
}

impl<T: Copy> AsRef<metal::BufferRef> for GPUVar<T> {
    #[inline]
    fn as_ref(&self) -> &metal::BufferRef {
        &self.inner
    }
}

impl<T: Copy> AsMut<metal::BufferRef> for GPUVar<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut metal::BufferRef {
        &mut self.inner
    }
}

impl<T: Copy> Clone for GPUVar<T> {
    fn clone(&self) -> Self {
        Self::with_value(&self.device, **self)
    }
}

impl<T: Copy> std::ops::Deref for GPUVar<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.as_ptr().as_ref().unwrap() }
    }
}

impl<T: Copy> std::ops::DerefMut for GPUVar<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.as_mut_ptr().as_mut().unwrap() }
    }
}

unsafe impl<T: Copy> Send for GPUVar<T> {}
unsafe impl<T: Copy> Sync for GPUVar<T> {}

impl<T: std::fmt::Debug + Copy> std::fmt::Debug for GPUVar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, f)
    }
}
