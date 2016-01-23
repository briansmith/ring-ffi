extern crate ring;

/// Moves `x` onto the heap and returns a mutable pointer to it.
fn heap_mut_ptr<T>(x: T) -> *mut T {
    Box::into_raw(Box::new(x))
}

/// Clones `*x` onto the heap and returns a mutable pointer to it.
unsafe fn clone_heap_mut_ptr<T: Clone>(x: *const T) -> *mut T {
    heap_mut_ptr((*x).clone())
}


macro_rules! wrap_algorithm {
    ( $wrapper_name:ident, $t:ty, $alg:expr ) => {
        /// Do NOT free the resulting pointer.
        #[no_mangle]
        pub unsafe fn $wrapper_name() -> *const $t {
            $alg as *const $t
        }
    }
}

wrap_algorithm!(ring_digest_algorithm_sha1, ring::digest::Algorithm,
                &ring::digest::SHA1);
wrap_algorithm!(ring_digest_algorithm_sha256, ring::digest::Algorithm,
                &ring::digest::SHA256);
wrap_algorithm!(ring_digest_algorithm_sha384, ring::digest::Algorithm,
                &ring::digest::SHA384);
wrap_algorithm!(ring_digest_algorithm_sha512, ring::digest::Algorithm,
                &ring::digest::SHA512);

/// Returns a pointer to a heap-allocated `ring::digest::Context`. You must
/// call `ring_digest_context_finish` to free the returned context.            
#[no_mangle]
pub unsafe fn ring_digest_context_new(algorithm: *const ring::digest::Algorithm)
                -> *mut ring::digest::Context {
    if algorithm.is_null() {
        return std::ptr::null_mut();
    }
    heap_mut_ptr(ring::digest::Context::new(&*algorithm))
}

/// Calls `ctx.update()` with the given data.
#[no_mangle]
pub unsafe fn ring_digest_context_update(ctx: *mut ring::digest::Context,
                                         p: *const u8, len: usize) {
    (*ctx).update(std::slice::from_raw_parts(p, len))
}

/// Frees a context created by `ring_digest_context_new`.
#[no_mangle]
pub unsafe fn ring_digest_context_finish(ctx: *mut ring::digest::Context,
                                         out: *mut u8, out_capacity: usize)
                                         -> usize {
    // We can't call `finish` on (*ctx). `x.finish()` consumes `x` but in the
    // FFI interface we can't rely on the caller honoring Rust's move semantics.
    let copy = (*ctx).clone();
    let digest = copy.finish();
    let digest = digest.as_ref();
    let len = digest.len();
    if len > out_capacity {
        return 0;
    }
    std::ptr::copy_nonoverlapping(digest.as_ptr(), out, len);
    len
}

#[no_mangle]
pub unsafe fn ring_digest_context_delete(ctx: *mut ring::digest::Context) {
    let _ = Box::from_raw(ctx);
}

/// Clones a digest context, returning a pointer to the clone.
#[no_mangle]
pub unsafe fn ring_digest_context_clone(ctx: *mut ring::digest::Context)
                                        -> *mut ring::digest::Context {
    clone_heap_mut_ptr(ctx)
}
