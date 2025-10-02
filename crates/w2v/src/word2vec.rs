use std::{
    fs,
    io::{self, Read},
};

use hashbrown::HashMap;
use ndarray::Array1;

const BUF_SIZE: usize = 1 << 20;
const FLOAT_NBYTES: usize = 4;
const RELOAD_POS: usize = (0.9 * BUF_SIZE as f64) as usize;

pub fn read_w2v_file(path: &str, normalize: bool) -> io::Result<HashMap<String, Array1<f32>>> {
    let mut reader = io::BufReader::new(fs::File::open(path)?);

    let mut buf = vec![0; BUF_SIZE];
    let mut num_buf = vec![];

    let mut nbuf = reader.read(&mut buf)?;
    let mut pos = 0;
    let mut mark = 0;

    while buf[pos] >= b'0' && buf[pos] <= b'9' {
        pos += 1;
        num_buf.push(buf[pos]);
    }

    let voc_size: usize = String::from_utf8_lossy(&buf[mark..pos]).parse().unwrap();

    pos += 1;
    mark = pos;
    while buf[pos] >= b'0' && buf[pos] <= b'9' {
        pos += 1;
    }

    let dim: usize = String::from_utf8_lossy(&buf[mark..pos]).parse().unwrap();

    let mut dict = HashMap::new();

    for _ in 0..voc_size {
        if pos > RELOAD_POS {
            nbuf -= pos;
            buf.copy_within(pos.., 0);
            let n = reader.read(&mut buf[nbuf..])?;
            if n > 0 {
                nbuf += n;
            }
            pos = 0;
        }
        if buf[pos] == b'\n' {
            pos += 1;
        }
        mark = pos;
        while buf[pos] != b' ' {
            pos += 1;
        }
        let w = String::from_utf8(buf[mark..pos].to_vec()).expect("valid utf-8");

        pos += 1;
        let mut v = vec![0f32; dim];
        for j in 0..dim {
            v[j] = bytes_to_float(&buf, pos);
            pos += FLOAT_NBYTES;
        }

        if normalize {
            todo!("normalize is not supported yet")
        }

        if dict.contains_key(&w) {
            // Warning?
        } else {
            dict.insert(w, Array1::from_vec(v));
        }
    }
    Ok(dict)
}

fn bytes_to_float(buf: &[u8], pos: usize) -> f32 {
    let b0 = (buf[pos] as u32) & 255;
    let b1 = (buf[pos + 1] as u32) & 255;
    let b2 = (buf[pos + 2] as u32) & 255;
    let b3 = (buf[pos + 3] as u32) & 255;
    let fi = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
    f32::from_bits(fi)
}
