use crate::r_crypt::WASM_MEMORY_BUFFER_SIZE;

pub fn wasm_read(wasm_buffer: *const u8, offset: &mut usize) -> Vec<u8> {
    let mut result = Vec::new();

    unsafe {
        let mut max: usize = (*wasm_buffer.add(*offset + 0) as usize) * 256
            + (*wasm_buffer.add(*offset + 1) as usize);

        if max > WASM_MEMORY_BUFFER_SIZE {
            return result;
        }

        *offset += 2;
        max += *offset;

        for i in *offset..max {
            result.push(*wasm_buffer.add(i));
        }
        *offset = max;
    }

    result
}

pub fn wasm_write(wasm_buffer: *mut u8, offset: &mut usize, data: Vec<u8>) -> Option<usize> {
    let len: usize = data.len();

    if *offset + len > WASM_MEMORY_BUFFER_SIZE - 2 {
        return None;
    } else {
        unsafe {
            *wasm_buffer.add(*offset) = (len / 256) as u8;
            *wasm_buffer.add(*offset + 1) = (len % 256) as u8;

            *offset += 2;

            js_sys::Uint8Array::from(data.as_slice()).raw_copy_to_ptr(wasm_buffer.add(*offset));
        }
        *offset += len;
    }

    Some(len)
}
