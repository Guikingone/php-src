struct ZendValue {
    lval: i64,
    double: f64,
    zend_refcounted: *mut ZendRefcounted,
}

struct ZendRefCountedH {
    refcount: u32,
    type_info: u32,
}

struct ZendRefcounted {
    gc: ZendRefCountedH,
}

struct ZendString {
    gc: ZendRefCountedH,
    h: u32,
    len: u32,
    val: [u8],
}
