use std::mem::size_of;

// change the type below to whatever you want your memory to be
// should implement Default
type MEMORY_TYPE = ();

enum Move {
    Cooperate,
    Defect,
}

fn run(histMine: &[Move], histTheirs: &[Move], memory: &mut MEMORY_TYPE, randomness: f64) -> Move {
    Move::Cooperate // write your own code here
}

// boring ffi, please ignore
#[export_name = "IPDABI_STEP"]
pub unsafe extern "C" fn IPDABI_STEP(
    histMine: *const u8,
    histTheirs: *const u8,
    len: u32,
    memory: *mut core::ffi::c_void,
    randomness: f64,
) -> *const core::ffi::c_void {
    let mut mem = 0 as *mut *mut MEMORY_TYPE;
    if memory.is_null() {
        mem = &mut(Box::leak(Box::<MEMORY_TYPE>::new(Default::default())) as *mut MEMORY_TYPE) as *mut *mut MEMORY_TYPE
    } else {
        mem = memory as *mut *mut MEMORY_TYPE
    }
    let histMineU8 = std::slice::from_raw_parts(histMine, len as usize);
    let histTheirsU8 = std::slice::from_raw_parts(histTheirs, len as usize);
    let histMineMove = histMineU8
        .iter()
        .map(|&a| {
            if a == 0 {
                Move::Defect
            } else {
                Move::Cooperate
            }
        })
        .collect::<Vec<_>>();
    let histTheirsMove = histTheirsU8
        .iter()
        .map(|&a| {
            if a == 0 {
                Move::Defect
            } else {
                Move::Cooperate
            }
        })
        .collect::<Vec<_>>();
    let result = run(
        &*histMineMove,
        &*histTheirsMove,
        std::mem::transmute(*mem),
        randomness,
    );
    let mut retval = std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(
        size_of::<*mut core::ffi::c_void>() + 1,
        8,
    ));
    std::ptr::write(
        retval as *mut *mut core::ffi::c_void,
        mem as *mut core::ffi::c_void,
    );
    std::ptr::write(
        retval.offset(size_of::<*mut core::ffi::c_void>() as isize),
        match result {
            Move::Cooperate => 1u8,
            Move::Defect => 0u8,
        },
    );
    retval as *const core::ffi::c_void
}

#[export_name = "IPDABI_MALLOC"]
pub unsafe extern "C" fn IPDABI_MALLOC(size: usize) -> *const core::ffi::c_void {
    if size == 0 {
        return 0 as *const core::ffi::c_void;
    }
    std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(size, 8))
        as *const core::ffi::c_void
}

#[export_name = "IPDABI_FREE"]
pub unsafe extern "C" fn IPDABI_FREE(ptr: *const core::ffi::c_void) {
    // let it leak for now
    // if you have a better impl make a PR
}
